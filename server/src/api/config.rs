use actix_web::web;

use super::calendar::{get_calendar_by_id, get_calendars_by_user_id};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_calendars_by_user_id);
    cfg.service(get_calendar_by_id);
}
