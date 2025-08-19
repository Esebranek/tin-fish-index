use actix_web::{Responder, get, web};
use serde::Serialize;

// Configure brand related handlers
pub fn brand_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_brands);
}

#[derive(Serialize)]
struct Brand {
    name: String,
}

#[derive(Serialize)]
struct BrandList {
    brands: Vec<Brand>,
}

// List brands
#[get("")]
async fn get_brands() -> impl Responder {
    let brand_list = BrandList { brands: Vec::new() };
    web::Json(brand_list)
}
