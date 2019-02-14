#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#UserActivationToken {
    #[serde(rename = "activationToken", skip_serializing_if = "Option::is_none")]
    r#activation_token: Option<String>,
    #[serde(rename = "activationUrl", skip_serializing_if = "Option::is_none")]
    r#activation_url: Option<String>,
}

impl r#UserActivationToken {
    pub fn new(
    ) -> Self {
        Self {
          r#activation_token: None,
          r#activation_url: None,
        }
    }

    pub fn set_activation_token(&mut self, r#activation_token: String) {
        self.r#activation_token = Some(r#activation_token);
    }

    pub fn with_activation_token(mut self, r#activation_token: String) -> Self {
        self.r#activation_token = Some(r#activation_token);
        self
    }

    pub fn r#activation_token(&self) -> Option<&str> {
        self.r#activation_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_activation_token(&mut self) {
        self.r#activation_token = None;
    }

    pub fn set_activation_url(&mut self, r#activation_url: String) {
        self.r#activation_url = Some(r#activation_url);
    }

    pub fn with_activation_url(mut self, r#activation_url: String) -> Self {
        self.r#activation_url = Some(r#activation_url);
        self
    }

    pub fn r#activation_url(&self) -> Option<&str> {
        self.r#activation_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_activation_url(&mut self) {
        self.r#activation_url = None;
    }
}
