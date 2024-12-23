use actix_web::{post, web, HttpResponse, Responder, cookie::Cookie};
use serde::Deserialize;
use crate::services::auth_service::AuthService;
#[derive(Deserialize)]
pub struct LoginRequest {
  username: String,
  password: String,
}

#[post("/login")]
pub async fn login(
  login_data: web::Json<LoginRequest>,
  auth_service: web::Data<AuthService>,
) -> impl Responder {
  // In a real app, validate credentials here
  if login_data.username == "admin" && login_data.password == "password" {
    let token = auth_service.create_session(login_data.username.clone());
    
    let cookie = Cookie::build("session", token)
      .http_only(true)
      .secure(true)
      .finish();

    HttpResponse::Ok()
      .cookie(cookie)
      .json(serde_json::json!({"message": "Logged in successfully"}))
  } else {
    HttpResponse::Unauthorized()
      .json(serde_json::json!({"message": "Invalid credentials"}))
  }
}