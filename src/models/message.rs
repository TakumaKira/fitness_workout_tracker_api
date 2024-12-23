use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::db::schema::messages;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = messages)]
pub struct Message {
  pub id: i32,
  pub message: String,
  pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
  pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
  pub message: String,
} 