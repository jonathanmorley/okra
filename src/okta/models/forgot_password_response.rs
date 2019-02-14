#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ForgotPasswordResponse {
    #[serde(rename = "resetPasswordUrl", skip_serializing_if = "Option::is_none")]
    r#reset_password_url: Option<String>,
}

impl r#ForgotPasswordResponse {
    pub fn new(
    ) -> Self {
        Self {
          r#reset_password_url: None,
        }
    }

    pub fn set_reset_password_url(&mut self, r#reset_password_url: String) {
        self.r#reset_password_url = Some(r#reset_password_url);
    }

    pub fn with_reset_password_url(mut self, r#reset_password_url: String) -> Self {
        self.r#reset_password_url = Some(r#reset_password_url);
        self
    }

    pub fn r#reset_password_url(&self) -> Option<&str> {
        self.r#reset_password_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_reset_password_url(&mut self) {
        self.r#reset_password_url = None;
    }
}
