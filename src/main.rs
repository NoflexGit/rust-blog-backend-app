use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use models::{NewPost, Post};
use schema::posts::dsl::*;
use std::fmt::Error;
use uuid::Uuid;

mod api;
mod models;
mod schema;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_posts(&self) -> Result<Vec<Post>, Error> {
        let todos = posts
            .load::<Post>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos");
        Ok(todos)
    }

    pub fn get_post(&self, post_id: String) -> Result<Post, Error> {
        let post = posts
            .find(post_id)
            .first(&mut self.pool.get().unwrap())
            .expect("Error loading todo");
        Ok(post)
    }

    pub fn create_post(&self, post: NewPost) -> Result<NewPost, Error> {
        let uuid = Uuid::new_v4().to_string();

        let post = NewPost {
            id: Some(uuid),
            title: post.title,
            body: post.body,
        };
        diesel::insert_into(posts)
            .values(&post)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(post)
    }

    pub fn delete_post(&self, post_id: String) -> Result<(), Error> {
        if posts
            .find(&post_id)
            .first::<Post>(&mut self.pool.get().unwrap())
            .is_err()
        {
            return Err(Error);
        }

        diesel::delete(posts.filter(id.eq(post_id)))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo");
        Ok(())
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
