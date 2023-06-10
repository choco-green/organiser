use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};

use crate::{model::auth_model::RegisterUserSchema, service::auth_service, AppState};

#[post("/auth/register")]
async fn register(body: Json<RegisterUserSchema>, data: Data<AppState>) -> impl Responder {
    auth_service::register(body, data).await
}
