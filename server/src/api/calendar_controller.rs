use actix_web::{get, web, Responder, error};
use sea_orm::{prelude::Uuid, EntityTrait, ModelTrait};
use serde_json::json;

use crate::{
    api::response_handler::{failed_to_find, failed_to_parse, handle_error, handle_success},
    entity::prelude::{Calendar, Event, User},
    service::calendar_service,
    AppState,
};

#[get("/calendar")]
pub async fn get_calendars(
    path: Option<web::Path<i32>>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Parse the user_id from the request
    let user_id = match path {
        Some(user_id) => user_id.into_inner(),
        None => return failed_to_parse("user id"),
    };

    // Find the user by the user_id
    let user = match User::find_by_id(user_id).one(&data.db).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return failed_to_find("user", "user id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Find all calendars related to the user
    let calendars = match user.find_related(Calendar).all(&data.db).await {
        Ok(calendars) => match !calendars.is_empty() {
            true => calendars,
            false => return failed_to_find("calendar", "user id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };
    // Return the calendars
    handle_success("calendars", calendars)
}

// todo: validate that the request user is the owner of the calendar
#[get("/calendar/{calendarId}")]
pub async fn get_calendar(
    path: Option<web::Path<i32>>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Parse the calendar_id from the request
    let calendar_id = match path {
        Some(id) => id.into_inner(),
        None => return failed_to_parse("calendar id"),
    };

    // Find the calendar by the calendar_id
    let calendar = match Calendar::find_by_id(calendar_id).one(&data.db).await {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return failed_to_find("calendar", "calendar id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Return the calendar
    handle_success("calendar", calendar)
}

#[get("/calendar/{calendarId}/events")]
pub async fn get_events(path: Option<web::Path<i32>>, data: web::Data<AppState>) -> impl Responder {
    // Parse the calendar_id from the request
    let calendar_id = match path {
        Some(id) => id.into_inner(),
        None => return failed_to_parse("calendar id"),
    };

    // Find the calendar by the calendar_id
    let calendar = match Calendar::find_by_id(calendar_id).one(&data.db).await {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return failed_to_find("calendar", "calendar id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Find all events related to the calendar
    let events = match calendar.find_related(Event).all(&data.db).await {
        Ok(events) => events,
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Return the events
    handle_success("events", events)
}

#[get("/calendar/{calendarId}/events/{eventId}")]
pub async fn get_event(
    path: Option<web::Path<(i32, i32)>>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Parse the calendar_id and event_id from the request
    let (calendar_id, event_id) = match path {
        Some(ids) => ids.into_inner(),
        None => return failed_to_parse("calendar id and event id"),
    };

    // Find the calendar by the calendar_id
    let calendar = match Calendar::find_by_id(calendar_id).one(&data.db).await {
        Ok(calendar) => match calendar {
            Some(calendar) => calendar,
            None => return failed_to_find("calendar", "calendar id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Find the event by the event_id
    let event: crate::entity::event::Model = match Event::find_by_id(event_id).one(&data.db).await {
        Ok(event) => match event {
            Some(event) => event,
            None => return failed_to_find("event", "calendar id"),
        },
        Err(DbErr) => return Err(error::ErrorInternalServerError(DbErr))
    };

    // Return the event
    handle_success("event", event)
}
