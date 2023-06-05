use actix_web::web;

use super::calendar::get_calendar;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_calendar);
}