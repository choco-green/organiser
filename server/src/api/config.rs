use actix_web::web;

use super::{
    calendar_controller::{get_calendar, get_calendars, get_event, get_events},
    user_controller::{get_user, get_user_with_query, get_users_with_query},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_calendar);
    cfg.service(get_calendars);
    cfg.service(get_event);
    cfg.service(get_events);
    cfg.service(get_user);
    cfg.service(get_user_with_query);
    cfg.service(get_users_with_query);
}
