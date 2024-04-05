use crate::models::{NewPost, Post};
use crate::Database;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/posts")]
pub async fn create_todo(db: web::Data<Database>, new_post: web::Json<NewPost>) -> HttpResponse {
    let todo = db.create_post(new_post.into_inner());
    match todo {
        Ok(new_post) => HttpResponse::Ok().json(new_post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/posts")]
pub async fn get_todos(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_posts();
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_todo(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let todo = db.delete_post(id);
    match todo {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/posts/{id}")]
pub async fn get_post(db: web::Data<Database>, path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let post = db.get_post(id);
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_todo)
            .service(delete_todo)
            .service(get_todos)
            .service(get_post),
    );
}
