
use actix_web::{web};
use crate::templates::index;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/").
        route(web::get().to(index),
    ));
}
