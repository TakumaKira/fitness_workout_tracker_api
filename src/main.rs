mod db;
mod handlers;
mod services;
mod models;
mod middleware;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();
  
  let pool = db::db::init_db()
    .expect("Failed to initialize database");

  let message_service = web::Data::new(services::message_service::MessageService::new(pool));
  let auth_service = web::Data::new(services::auth_service::AuthService::new());

  HttpServer::new(move || {
    App::new()
      .app_data(message_service.clone())
      .app_data(auth_service.clone())
      .wrap(actix_web::middleware::Logger::default())
      .service(handlers::auth::login)
      .service(
        web::scope("/api")
          .wrap(middleware::auth::Auth)
          .service(handlers::message::hello)
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
