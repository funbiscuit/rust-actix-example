use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::actix::Addr;
use crate::DbActor;
use crate::schema::articles;

pub struct AppState {
    pub db: Addr<DbActor>
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleData{
    pub title: String,
    pub body: String,
}
