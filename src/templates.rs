
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
//use crate::schema::users::dsl::*;

#[derive(Serialize, Deserialize,)]
pub struct Register {
    username: String,
    email: String,
    password: String,
    confirmpassword: String,
}
pub fn insert_new_user <'a>(
    nu: &'a UserInsertable,
    conn: &PgConnection
,
) -> User {
    
    use crate::schema::users;
    


    

    let ret_val = diesel::insert_into(users::table).values(nu).get_result(conn).expect("error");
    return ret_val;
}
/// Simple handle POST request
/// https://cloudmaker.dev/authenticate-api-users/
pub async fn register(params: web::Form<Register>,pool: web::Data<DbPool>) -> Result<HttpResponse,Error> {
    println!("username:{}",params.username);
    println!("password:{}",params.password);

    let conn = pool.get()
        .map_err(|_| error::ErrorInternalServerError("database connection error"))?;

    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    let hashedpass = argon2::hash_encoded(params.password.as_bytes(), &salt, &config)
    .map_err(|e| error::ErrorInternalServerError("password wrap error"))?;
    println!("hashedpassword:{}",hashedpass);
    println!("confirmpassword:{}",params.confirmpassword);
    let verify = argon2::verify_encoded(&hashedpass, &params.password.as_bytes());
    let naive_date_time = chrono::Utc::now().naive_utc();

    let new_user = UserInsertable {
        email:params.email.clone(),
        password:hashedpass,
        username:params.username.clone(),
        last_login:naive_date_time.clone(),
        created_on:naive_date_time.clone(),

    };
    let user = insert_new_user(&new_user,&conn);
    println!("userid::{}",user.user_id);
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

        // let name = query.get("username");
        // if name.is_none(){
        //     let e =  Err(error::ErrorInternalServerError("query param error"));
        //     return e;
        // }
        // let name = name.unwrap();
       



        let s = if let Some(name) = query.get("username") {
            // submitted form
            println!("username:{}",name);
            let mut ctx = tera::Context::new();
            ctx.insert("name", name);
            ctx.insert("text", "Welcome! Create User");
             tmpl.render("user.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        } else {
            println!("didn't work");
            tmpl.render("index.html", &tera::Context::new())
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        };
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



    