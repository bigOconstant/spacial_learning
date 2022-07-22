
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
pub struct RegisterCheck{
    username:bool,
    email:bool,
    password:bool,
    confirmpassword:bool
}

impl RegisterCheck {
    fn new()->RegisterCheck {
        return RegisterCheck { username: true, email: true, password: true, confirmpassword: true };
    }
}


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
) -> Result<User, diesel::result::Error> {
    
    use crate::schema::users;
    


    

    let ret_val = diesel::insert_into(users::table).values(nu).get_result(conn);
    return ret_val;
}
/// Simple handle POST request
/// https://cloudmaker.dev/authenticate-api-users/
pub async fn register(params: web::Form<Register>,pool: web::Data<DbPool>,tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    let mut check = RegisterCheck::new();
    if params.username.is_empty() {
        check.username = false;
        // let retval = build_register_page(&check,tmpl).await;
        // return retval;
    }
    if params.email.is_empty() {
        check.email = false;
    }
    if params.password.is_empty() {
        check.password = false;
    }
    if params.confirmpassword.is_empty() {
        check.confirmpassword = false;
    }

    if !check.email || !check.password || !check.confirmpassword || !check.username {
        let retval = build_register_page(&check,tmpl).await;
        return retval;
    }
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
    let user_result = insert_new_user(&new_user,&conn);
    match user_result {
        Ok(user)=>{
            println!("new user created");
        },
        Err(x)=>{
            println!("Error:{}",x);
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
     

   
        let retval =  build_register_page(&check,tmpl).await;
        return retval;
    }

    pub async fn build_register_page(rg:&RegisterCheck,tmpl: web::Data<tera::Tera>)-> Result<HttpResponse, Error>  {
        let mut ctx = tera::Context::new();
        ctx.insert("check", &rg);
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



    