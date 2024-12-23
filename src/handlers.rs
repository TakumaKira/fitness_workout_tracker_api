use actix_web::{get, HttpResponse, Responder};
use crate::models::MessageResponse;

#[get("/")]
pub async fn hello() -> impl Responder {
    let response = MessageResponse {
        message: "Hello, world!".to_string(),
    };
    HttpResponse::Ok().json(response)
} 