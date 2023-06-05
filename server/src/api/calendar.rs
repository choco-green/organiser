use actix_web::{get, web, Responder};
use sea_orm::{EntityTrait, ModelTrait};
use serde_json::json;

use crate::{
    api::generic::{handle_error, handle_fail, handle_success},
    entity::prelude::{Calendar, User},
    AppState,
};

#[get("/calendar/user/{userId}")]
pub async fn get_calendars_by_user_id(
    user_id: Option<web::Path<i32>>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Parse the user_id from the request
    let user_id = match user_id {
        Some(user_id) => user_id.into_inner(),
        None => return handle_fail("UserId failed to parse"),
    };

    let conn = &data.db;

    // Find the user by the user_id
    let user = User::find_by_id(user_id).one(conn).await;
    let user = match user {
        Ok(user) => match user {
            Some(user) => user,
            None => return handle_fail(format!("User with Id: {user_id} not found")),
        },
        Err(e) => return handle_error(e),
    };

    // Find all calendars related to the user
    let calendars = user.find_related(Calendar).all(conn).await;
    let calendars = match calendars {
        Ok(calendars) => calendars,
        Err(e) => return handle_error(e),
    };

    // Return the calendars
    handle_success(json!({ "calendars": calendars }))
}

#[get("/calendar/{calendarId}")]
pub async fn get_calendar_by_id(
    calendar_id: Option<web::Path<i32>>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Parse the calendar_id from the request
    let calendar_id = match calendar_id {
        Some(id) => id.into_inner(),
        None => return handle_fail("UserId failed to parse"),
    };

    let conn = &data.db;

    // Find the calendar by the calendar_id
    let calendar = Calendar::find_by_id(calendar_id).one(conn).await;
    let calendar = match calendar {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return handle_fail(format!("Calendar with ID: {calendar_id} not found")),
        },
        Err(e) => return handle_error(e),
    };

    // Return the calendar
    handle_success(json!({ "calendar": calendar }))
}
