use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{NewMessage, MessageResponse};
use crate::schema::messages;

pub struct MessageService {
  pool: DbPool,
}

impl MessageService {
  pub fn new(pool: DbPool) -> Self {
    Self { pool }
  }

  pub fn create_message(&self, message: String) -> Result<String, diesel::result::Error> {
    let mut conn = self.pool.get().unwrap();
    let new_message = NewMessage { message };
    
    diesel::insert_into(messages::table)
      .values(&new_message)
      .returning(messages::message)
      .get_result::<String>(&mut conn)
  }
}