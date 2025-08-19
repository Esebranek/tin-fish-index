use actix_web::{get, web, Responder};
use serde::Serialize;

// Configure health related handlers
pub fn health_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_up);
}

// Get status
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
