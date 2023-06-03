pub mod users;
use actix_web::{web::{Data,Json}, web, post, get, put, delete, HttpResponse};
use crate::{models::users::User, repository::database::Database};

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[post("/users")]
pub async fn create_user(db: Data<Database>, new_user: Json<User>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.get_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/users/{id}")]
pub async fn update_user_by_id(db: web::Data<Database>, id: web::Path<String>, updated_user: web::Json<User>) -> HttpResponse {
    let user = db.update_user_by_id(&id, updated_user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.delete_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(create_user)
            .service(get_users)
            .service(get_user_by_id)
            .service(update_user_by_id)
            .service(delete_user_by_id)
    );
}