#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#CreateSessionRequest {
    #[serde(rename = "sessionToken", skip_serializing_if = "Option::is_none")]
    r#session_token: Option<String>,
}

impl r#CreateSessionRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#session_token: None,
        }
    }

    pub fn set_session_token(&mut self, r#session_token: String) {
        self.r#session_token = Some(r#session_token);
    }

    pub fn with_session_token(mut self, r#session_token: String) -> Self {
        self.r#session_token = Some(r#session_token);
        self
    }

    pub fn r#session_token(&self) -> Option<&str> {
        self.r#session_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_session_token(&mut self) {
        self.r#session_token = None;
    }
}
