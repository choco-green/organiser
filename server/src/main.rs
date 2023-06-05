use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

mod api;
mod entity;

use crate::api::config::config;
#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection =
        Database::connect("postgres://superuser:superpassword@localhost/organiser")
            .await
            .unwrap();

    let state = AppState { db };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
