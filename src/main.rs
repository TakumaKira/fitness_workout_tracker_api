mod models;
mod handlers;
mod db;
mod schema;
mod services;

use actix_web::{App, HttpServer, web};
use handlers::hello;
use crate::services::MessageService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();
  
  let pool = db::init_db()
    .expect("Failed to initialize database");

  let message_service = web::Data::new(MessageService::new(pool));

  HttpServer::new(move || {
    App::new()
      .app_data(message_service.clone())
      .service(hello)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
    .await
}
