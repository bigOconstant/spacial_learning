
use actix_web::{error, web,  Error, HttpResponse, Result};

use std::collections::HashMap;

// store tera template in application state
    pub async fn index(
        tmpl: web::Data<tera::Tera>,
        query: web::Query<HashMap<String, String>>,
    ) -> Result<HttpResponse, Error> {
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

