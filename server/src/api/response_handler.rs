use actix_web::{Error, HttpResponse};
use serde::Serialize;
use serde_json::json;

pub fn handle_error(err: Error) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": "error",
        "code": err.as_response_error().status_code().to_string(),
        "message": format!("{:?}", err)
    }))
}

pub fn handle_conflict(model: &str, column: &str) -> HttpResponse {
    HttpResponse::Conflict().json(json!({
        "status": "fail",
        "message": format!("{model} with that {column} already exists")
    }))
}

pub fn handle_success<T: Serialize>(field: &str, data: T) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": json!({ field: data })
    }))
}

pub fn failed_to_parse(field: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": "fail",
        "message": format!("Failed to parse {field}")
    }))
}

pub fn failed_to_find(target: &str, field: &str) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "status": "fail",
        "message": format!("{target} not found by {field}")
    }))
}

pub fn not_logged_in() -> HttpResponse {
    HttpResponse::Unauthorized().json(json!({
        "status": "fail",
        "message": "Token is not provided in the request"
    }))
}
