use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

async fn index() -> impl Responder {
    "Hello, world"
}

mod posts;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/posts", web::post().to(posts::create_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
