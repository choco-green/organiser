use actix_web::HttpResponse;
use sea_orm::DbErr;
use serde_json::json;

pub fn handle_error(err: DbErr) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": "error",
        "message": format!("{:?}", err)
    }))
}

pub fn handle_fail<S: AsRef<str>>(message: S) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": "fail",
        "message": message.as_ref()
    }))
}

pub fn handle_success(data: serde_json::Value) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": data
    }))
}
