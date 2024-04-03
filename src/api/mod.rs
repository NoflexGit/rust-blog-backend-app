use crate::models::{NewPost, Post};
use crate::Database;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/posts")]
pub async fn create_todo(db: web::Data<Database>, new_post: web::Json<NewPost>) -> HttpResponse {
    let todo = db.create_todo(new_post.into_inner());
    match todo {
        Ok(new_post) => HttpResponse::Ok().json(new_post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(create_todo));
}
