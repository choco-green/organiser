use actix_web::{Responder, get, HttpResponse, web};
use sea_orm::{EntityTrait, ModelTrait};
use serde_json::json;

use crate::{entity::prelude::{Calendar, User}, AppState};

#[get("/calendar/user/{userId}")]
pub async fn get_calendar(path: Option<web::Path<i32>>, data: web::Data<AppState>) -> impl Responder {
    let id = match path {
        Some(path) => path.into_inner(),
        None => return HttpResponse::InternalServerError()
            .json(json!({
                "status": "error", "message": "User ID failed to parse"
            })),
    };

    let conn = &data.db;

    let user = User::find_by_id(id)
        .one(conn)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(err) => return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": format!("{:?}", err)})),
    };

    let user = match user {
        Some(user) => user,
        None => return HttpResponse::InternalServerError()
            .json(json!({
                "status": "fail",
                "message": format!("User with ID: {} not found", id),
            })),
    };

    let calendars = user
        .find_related(Calendar)
        .all(conn)
        .await;

    match calendars {
        Ok(calendars) => return HttpResponse::Ok()
            .json(json!({
                "status": "success", "data": json!({ "calendars": calendars }),
            })),
        Err(err) => return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": format!("{:?}", err)})),
    }
}