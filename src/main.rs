use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::{connection, prelude::*};
use dotenv::dotenv;
use models::{NewPost, Post};
use schema::posts::dsl::*;
use std::env;
use std::fmt::Error;

mod api;
mod models;
mod schema;

type DBPoolSqlite = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pool: DBPoolSqlite,
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool: DBPoolSqlite = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        posts
            .load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_todo(&self, post: NewPost) -> Result<NewPost, Error> {
        let post = NewPost {
            author_id: post.author_id,
            title: post.title,
            content: post.content,
        };
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(post)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_db = Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::config)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
