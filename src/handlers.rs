use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{Message, NewMessage, MessageResponse};
use crate::schema::messages;

#[post("/")]
pub async fn hello(
    message_data: web::Json<MessageResponse>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(MessageResponse {
                message: "Database connection error".to_string(),
            })
        }
    };

    let new_message = NewMessage {
        message: message_data.message.clone(),
    };

    let result = web::block(move || {
        diesel::insert_into(messages::table)
            .values(&new_message)
            .returning(messages::message)
            .get_result::<String>(&mut conn)
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