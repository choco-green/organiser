use actix_web::{
    get,
    web::{Data, Path, Query},
    Responder,
};

use crate::{
    service::user_service,
    model::user_model::Filter as UserFilter,
    AppState,
};

#[get("/user/{user_id}}")]
pub async fn get_user(path: Option<Path<i32>>, data: Data<AppState>) -> impl Responder {
    user_service::get_user(path, data).await
}

#[get("/user")]
pub async fn get_user_with_query(
    query: Option<Query<UserFilter>>,
    data: Data<AppState>,
) -> impl Responder {
    user_service::get_user_with_query(query, data).await
}

#[get("/users")]
pub async fn get_users_with_query(
    query: Option<Query<UserFilter>>,
    data: Data<AppState>,
) -> impl Responder {
    user_service::get_users_with_query(query, data).await
}
