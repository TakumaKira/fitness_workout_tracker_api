use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

pub struct AuthService {
  sessions: RwLock<HashMap<String, String>>, // token -> user_id
}

impl AuthService {
  pub fn new() -> Self {
    Self {
      sessions: RwLock::new(HashMap::new()),
    }
  }

  pub fn create_session(&self, user_id: String) -> String {
    let token = Uuid::new_v4().to_string();
    self.sessions.write().unwrap().insert(token.clone(), user_id);
    token
  }

  pub fn validate_session(&self, token: &str) -> bool {
    self.sessions.read().unwrap().contains_key(token)
  }

  pub fn remove_session(&self, token: &str) {
    self.sessions.write().unwrap().remove(token);
  }
}