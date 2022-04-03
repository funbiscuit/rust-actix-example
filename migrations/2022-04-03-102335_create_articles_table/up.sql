
create table articles
(
    uuid      uuid primary key,
    title     varchar not null,
    body      text    not null,
    published boolean not null default 'f'
);
