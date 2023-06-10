use actix_web::web::{Data, Path, Query};
use actix_web::HttpResponse;

use crate::{
    api::response_handler::{handle_error, handle_success},
    model::user_model::{find_many_by_query, find_one_by_id, Filter as UserFilter},
    AppState,
};

pub async fn get_user(user_id: Option<Path<i32>>, data: Data<AppState>) -> HttpResponse {
    match find_one_by_id(user_id, data).await {
        Ok(user) => return handle_success("user", user),
        Err(err) => return handle_error(err),
    };
}

pub async fn get_user_with_query(
    query: Option<Query<UserFilter>>,
    data: Data<AppState>,
) -> HttpResponse {
    match find_many_by_query(query, data).await {
        Ok(user) => return handle_success("user", user.first()),
        Err(err) => return handle_error(err),
    };
}

pub async fn get_users_with_query(
    query: Option<Query<UserFilter>>,
    data: Data<AppState>,
) -> HttpResponse {
    match find_many_by_query(query, data).await {
        Ok(users) => return handle_success("users", users),
        Err(err) => return handle_error(err),
    };
}
