mod models;
mod handlers;
mod db;
mod schema;

use actix_web::{App, HttpServer, web};
use handlers::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let pool = db::init_db()
        .expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
