
use actix_web::{error, web,  Error, HttpResponse, Result};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use argon2::Config;
use rand::Rng;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
  };
type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;
use chrono::{NaiveDateTime};

use crate::diesel::RunQueryDsl;

use crate::models::*;
use crate::view_models::*;
//use crate::schema::users::dsl::*;





pub fn insert_new_user <'a>(
    nu: &'a UserInsertable,
    conn: &PgConnection
,
) -> Result<User, diesel::result::Error> {
    
    use crate::schema::users;
    


    

    let ret_val = diesel::insert_into(users::table).values(nu).get_result(conn);
    return ret_val;
}
/// Simple handle POST request
/// https://cloudmaker.dev/authenticate-api-users/
pub async fn register(mut params: web::Form<Register>,pool: web::Data<DbPool>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    


    let mut check = params.set_error();

    if check.has_error() {
        params.password = "".to_string();
        params.confirmpassword = "".to_string();
        let retval = build_register_page(&check,&params,tmpl).await;
        return retval; 
    }
 
    let conn = pool.get()
        .map_err(|_| error::ErrorInternalServerError("database connection error"))?;

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    let hashedpass = argon2::hash_encoded(params.password.as_bytes(), &salt, &config)
    .map_err(|e| error::ErrorInternalServerError("password wrap error"))?;
    
    let verify = argon2::verify_encoded(&hashedpass, &params.password.as_bytes());
    let naive_date_time = chrono::Utc::now().naive_utc();

    let new_user = UserInsertable {
        email:params.email.clone(),
        password:hashedpass,
        username:params.username.clone(),
        last_login:naive_date_time.clone(),
        created_on:naive_date_time.clone(),

    };
    let user_result = insert_new_user(&new_user,&conn);
    match user_result {
        Ok(user)=>{
            println!("new user created");
        },
        Err(x)=> {
            if x.to_string() == "duplicate key value violates unique constraint \"users_username_key\"".to_string() { //Kinda hacky but also pretty effecient
                check.set_username_error("username taken");
            } else if x.to_string() == "duplicate key value violates unique constraint \"users_email_key\"".to_string() {
                check.set_email_error("email taken");
            }
            println!("Error:{}",x);
            let retval = build_register_page(&check,&params,tmpl).await;
            return retval; 
            
        }
    }


    //println!("userid::{}",user.user_id);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.username)))
}

//pub async fn save_new_user()

pub async fn loggedin(
    tmpl: web::Data<tera::Tera>,
    _: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {



        println!("didn't work");
       let render =  tmpl.render("loggedin.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
   
    Ok(HttpResponse::Ok().content_type("text/html").body(render))
}



// store tera template in application state
    pub async fn index(
        tmpl: web::Data<tera::Tera>,
        query: web::Query<HashMap<String, String>>,
    ) -> Result<HttpResponse, Error> {

       
        let check = RegisterCheck::new();
     
        let form = Register::new();
   
        let retval =  build_register_page(&check,&form,tmpl).await;
        return retval;
    }

    pub async fn build_register_page(rg:&RegisterCheck,f:&Register,tmpl: web::Data<tera::Tera>)-> Result<HttpResponse, Error>  {
        let mut ctx = tera::Context::new();
        ctx.insert("check", &rg);
        ctx.insert("f", &f);
        let s = tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?;
       
        Ok(HttpResponse::Ok().content_type("text/html").body(s))

    }


    pub async fn home(
        tmpl: web::Data<tera::Tera>,
        query: web::Query<HashMap<String, String>>,
    ) -> Result<HttpResponse, Error> {  
        let s =  tmpl.render("home.html", &tera::Context::new())
                .map_err(|_| error::ErrorInternalServerError("Template error"))?;
 
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }



    