use actix_web::{error, web,  Error, HttpResponse, Result,cookie::Cookie};
use uuid::Uuid;

use diesel::{
    r2d2::{ConnectionManager},
    PgConnection,
  };
type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::viewmodels::login::{*, self};

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

pub async fn build_login_page_cookie(cookie_string: String,lc:&LoginCheck,l:&Login,tmpl: web::Data<tera::Tera>)-> Result<HttpResponse, Error>  {
  let mut ctx = tera::Context::new();
  //Todo; stick this into session table;
  ctx.insert("check", &lc);
  ctx.insert("f", &l);
  let s = tmpl.render("login.html", &ctx)
          .map_err(|_| error::ErrorInternalServerError("Template error"))?;
 
  let cookie = Cookie::build("session",cookie_string).finish();

  Ok(HttpResponse::Ok().cookie(cookie).content_type("text/html").body(s))
}

pub async fn cooke_return(cookie_string: String, u : crate::crudmodels::user::User,pool: web::Data<DbPool>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
  let conn = pool.get()
  .map_err(|_| error::ErrorInternalServerError("database connection error"))?;
  use crate::crudmodels::user_sessions::UserSessionInsertable;
  
  let new_user = UserSessionInsertable {
    user_id:u.user_id,
    uuid:cookie_string.clone(),
  };

  let session_result = crate::crud::user_session::insert_new_user_session(&new_user, &conn);

  match session_result {
    Ok(session)=>{
        let retval = build_login_page_cookie(cookie_string.clone(),&LoginCheck::new(),&login::Login::new(), tmpl).await;
        return retval;
        //println!("new user created");
    },
    Err(x)=> {
        // todo, return some kind of error 
        let s = tmpl.render("error_generic.html", &tera::Context::new())
          .map_err(|_| error::ErrorInternalServerError("Template error"))?;
 
  //let cookie = Cookie::build("session",cookie_string).finish();

      return Ok(HttpResponse::Ok().content_type("text/html").body(s))
        
    }
  }
}


pub async fn login_post(mut params: web::Form<Login>,pool: web::Data<DbPool>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {

    let mut check = params.set_error();

    if check.has_error() {
      params.password = "".to_string();
      let retval = build_login_page(&check,&params,tmpl).await;
      return retval; 
    }

    let conn = pool.get()
        .map_err(|_| error::ErrorInternalServerError("database connection error"))?;

    let user_result = crate::crud::login::login(&params.username, &conn);
    
    match user_result {
      Ok(u)=>{
        println!("login success?{}",u.user_id);
        let verify = argon2::verify_encoded(&&u.password, &params.password.as_bytes()).unwrap();
        if !verify {
          check.set_password_error("password incorrect");
          //println!("error:{}",e);
          let ret_val = build_login_page(&check,&params,tmpl).await;
          return ret_val;
        } else{
          println!("Password correct!");
          let cookie_string = Uuid::new_v4().to_string();
          let ret_val = cooke_return(cookie_string,u,pool,tmpl).await;
          return ret_val;
        }
      },
      Err(e) =>{// todo; change this 
        check.set_username_error("Something was incorrect");
        check.set_password_error("Something was incorrect");
        println!("error:{}",e);
        let ret_val = build_login_page(&check,&params,tmpl).await;
        return ret_val;
      }
        
    }
        
        //register::insert_new_user(&new_user,&conn);

}