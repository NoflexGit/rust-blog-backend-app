use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: Option<i32>,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub published_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}
