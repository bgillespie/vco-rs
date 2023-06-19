//! Structs used for login.

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

use crate::REDACTED;

/// `LoginAuth` is used for username/password cookie-based auth.
#[derive(Serialize, Deserialize)]
pub struct AuthObject {
    username: String,
    password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    password2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl Debug for AuthObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthObject({}, {})", self.username, REDACTED)
    }
}

impl AuthObject {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            password2: None,
            email: None,
        }
    }
}
