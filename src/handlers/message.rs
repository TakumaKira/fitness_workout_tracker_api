use actix_web::{post, web, HttpResponse, Responder};
use crate::services::message_service::MessageService;
use crate::models::message::MessageResponse;

#[post("/hello")]
pub async fn hello(
  message_data: web::Json<MessageResponse>,
  service: web::Data<MessageService>,
) -> impl Responder {
  let result = web::block(move || {
    service.create_message(message_data.message.clone())
  })
  .await;

  match result {
    Ok(result) => match result {
      Ok(message) => HttpResponse::Ok().json(MessageResponse { message }),
      Err(_) => HttpResponse::InternalServerError().json(MessageResponse {
        message: "Error creating message".to_string(),
      }),
    },
    Err(_) => HttpResponse::InternalServerError().json(MessageResponse {
      message: "Error creating message".to_string(),
    }),
  }
}