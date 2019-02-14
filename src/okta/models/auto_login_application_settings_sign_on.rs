#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AutoLoginApplicationSettingsSignOn {
    #[serde(rename = "loginUrl", skip_serializing_if = "Option::is_none")]
    r#login_url: Option<String>,
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    r#redirect_url: Option<String>,
}

impl r#AutoLoginApplicationSettingsSignOn {
    pub fn new(
    ) -> Self {
        Self {
          r#login_url: None,
          r#redirect_url: None,
        }
    }

    pub fn set_login_url(&mut self, r#login_url: String) {
        self.r#login_url = Some(r#login_url);
    }

    pub fn with_login_url(mut self, r#login_url: String) -> Self {
        self.r#login_url = Some(r#login_url);
        self
    }

    pub fn r#login_url(&self) -> Option<&str> {
        self.r#login_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login_url(&mut self) {
        self.r#login_url = None;
    }

    pub fn set_redirect_url(&mut self, r#redirect_url: String) {
        self.r#redirect_url = Some(r#redirect_url);
    }

    pub fn with_redirect_url(mut self, r#redirect_url: String) -> Self {
        self.r#redirect_url = Some(r#redirect_url);
        self
    }

    pub fn r#redirect_url(&self) -> Option<&str> {
        self.r#redirect_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_redirect_url(&mut self) {
        self.r#redirect_url = None;
    }
}
