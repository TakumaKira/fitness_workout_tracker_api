use diesel::prelude::*;
use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::schema::messages;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub message: &'a str,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
} 