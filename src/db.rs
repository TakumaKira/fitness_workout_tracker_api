use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::error::Error;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db() -> Result<DbPool, Box<dyn Error>> {
    let database_url = dotenvy::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;
    Ok(pool)
} 