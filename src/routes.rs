
use actix_web::{web};
use crate::template_logic::register::{index,register,home};
use crate::template_logic::login::{login_load,login_post};



pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("")
    .service(web::resource("/").route(web::get().to(home)))
    .service(
        web::resource("/register")
        .route(web::post().to(register))
        .route(web::get().to(index)
        ))
    .service(
        web::resource("/login")
        .route(web::post().to(login_post))
        .route(web::get().to(login_load)
    )));
}
