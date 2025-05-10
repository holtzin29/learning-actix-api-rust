use actix_web::{get, web, Responder};

use crate::utils::api_response;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello"))
}

#[get("/test")]
pub async fn test_greet() -> impl Responder {
    api_response::ApiResponse::new(200, "Test".to_string())
}
