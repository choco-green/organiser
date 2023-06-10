use actix_web::{web, HttpResponse};

use crate::api::response_handler::failed_to_parse;

pub async fn get_calendars(user_id: Option<web::Path<i32>>) -> Option<HttpResponse> {
    // Parse the user_id from the request
    let user_id = match user_id {
        Some(user_id) => user_id.into_inner(),
        None => return Some(failed_to_parse("user id")),
    };

    Some(failed_to_parse("user id"))
}
