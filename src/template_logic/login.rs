use actix_web::{error, web,  Error, HttpResponse, Result};

use std::collections::HashMap;
use argon2::Config;
use rand::Rng;
use diesel::{
    r2d2::{ConnectionManager},
    PgConnection,
  };
type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;


use crate::viewmodels::login::*;


pub async fn login_load(tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {

    let l = Login::new();
    let lc = LoginCheck::new();

    let retval =  build_login_page(&lc,&l,tmpl).await;
    return retval;
}

pub async fn build_login_page(lc:&LoginCheck,l:&Login,tmpl: web::Data<tera::Tera>)-> Result<HttpResponse, Error>  {
  let mut ctx = tera::Context::new();
  ctx.insert("check", &lc);
  ctx.insert("f", &l);
  let s = tmpl.render("login.html", &ctx)
          .map_err(|_| error::ErrorInternalServerError("Template error"))?;
 
  Ok(HttpResponse::Ok().content_type("text/html").body(s))

}

pub async fn login_post(mut params: web::Form<Login>,pool: web::Data<DbPool>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {

    let mut check = params.set_error();

    if check.has_error() {
      params.password = "".to_string();
      let retval = build_login_page(&check,&params,tmpl).await;
      return retval; 
    }else {
      let ret_val = build_login_page(&check,&params,tmpl).await;
      return ret_val; 
    }
}