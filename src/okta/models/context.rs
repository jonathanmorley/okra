#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Context {
    #[serde(rename = "deviceToken", skip_serializing_if = "Option::is_none")]
    r#device_token: Option<String>,
}

impl r#Context {
    pub fn new(
    ) -> Self {
        Self {
          r#device_token: None,
        }
    }

    pub fn set_device_token(&mut self, r#device_token: String) {
        self.r#device_token = Some(r#device_token);
    }

    pub fn with_device_token(mut self, r#device_token: String) -> Self {
        self.r#device_token = Some(r#device_token);
        self
    }

    pub fn r#device_token(&self) -> Option<&str> {
        self.r#device_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_device_token(&mut self) {
        self.r#device_token = None;
    }
}
