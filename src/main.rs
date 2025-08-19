mod routes;

use actix_web::{App, HttpServer, Responder, get, web};
use actix_web::middleware::Logger;
use log::info;
use serde::Serialize;

// Main method
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logging
    env_logger::init();

    info!("Application starting");

    // Start web server on localhost:8080
    HttpServer::new(|| App::new()
        // Access logging via Logger middleware
        .wrap(Logger::default())
        // Status handler
        .service(get_up)
        // Configure brand handlers
        .service(web::scope("/brands").configure(routes::brands::brand_config))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await.expect("Web server error");

    info!("Application stopping");
    Ok(())
}

#[get("/up")]
async fn get_up() -> impl Responder {
    let status = Status {
        status: String::from("happy"),
    };
    web::Json(status)
}

#[derive(Serialize)]
struct Status {
    status: String,
}
