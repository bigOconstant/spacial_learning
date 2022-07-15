use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{ middleware, web, App, HttpResponse, HttpServer, Result};
use tera::Tera;
use std::env;


use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
  };

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder()

    .max_size(5)// To do, put this in a config somewhere
    .build(manager)
    .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));


    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new( move|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(spacial_learning::routes::routes)
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}