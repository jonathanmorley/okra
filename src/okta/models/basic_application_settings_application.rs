#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#BasicApplicationSettingsApplication {
    #[serde(rename = "authURL", skip_serializing_if = "Option::is_none")]
    r#auth_url: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    r#url: Option<String>,
}

impl r#BasicApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#auth_url: None,
          r#url: None,
        }
    }

    pub fn set_auth_url(&mut self, r#auth_url: String) {
        self.r#auth_url = Some(r#auth_url);
    }

    pub fn with_auth_url(mut self, r#auth_url: String) -> Self {
        self.r#auth_url = Some(r#auth_url);
        self
    }

    pub fn r#auth_url(&self) -> Option<&str> {
        self.r#auth_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auth_url(&mut self) {
        self.r#auth_url = None;
    }

    pub fn set_url(&mut self, r#url: String) {
        self.r#url = Some(r#url);
    }

    pub fn with_url(mut self, r#url: String) -> Self {
        self.r#url = Some(r#url);
        self
    }

    pub fn r#url(&self) -> Option<&str> {
        self.r#url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_url(&mut self) {
        self.r#url = None;
    }
}
