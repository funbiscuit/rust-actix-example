extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix::SyncArbiter;
use actix_web::{App, delete, get, HttpRequest, HttpResponse, HttpServer, patch, post, put, Responder, web::{self, Data, Json, Path}};
use actix_web::dev::AppConfig;
use uuid::Uuid;

use actors::db::{Create, DbActor, Delete, GetArticles, Publish, Update};
use db_utils::{get_pool, run_migrations};
use models::AppState;

use crate::models::{Article, ArticleData};

mod models;
mod db_utils;
mod schema;
mod actors;

#[post("/new")]
async fn create_article(article: Json<ArticleData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let article = article.into_inner();

    match db.send(Create { title: article.title, body: article.body }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[post("/{id}/publish")]
async fn publish_article(id: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let id = id.into_inner();

    match db.send(Publish { uuid: id }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[delete("/{id}")]
async fn delete_article(id: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let id = id.into_inner();

    match db.send(Delete { uuid: id }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[put("/{id}")]
async fn update_article(id: Path<Uuid>, article: Json<ArticleData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let article = article.into_inner();
    let id = id.into_inner();

    match db.send(Update { uuid: id, title: article.title, body: article.body }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[get("/published")]
async fn get_published(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetArticles{}).await {
        Ok(Ok(articles)) => HttpResponse::Ok().json(articles),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(get_published)
            .service(delete_article)
            .service(publish_article)
            .service(create_article)
            .service(update_article)
            .data(AppState{
                db: db_addr.clone()
            })
    })
        .bind(("0.0.0.0", 4000))?
        .run()
        .await
}
