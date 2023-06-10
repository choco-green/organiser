use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use helper::jwt::Jwt;
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod api;
mod entity;
mod helper;
mod model;
mod service;

use crate::api::config::config;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    jwt: Jwt,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        db: Database::connect(env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
        jwt: Jwt::init(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
