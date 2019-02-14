#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SwaApplicationSettingsApplication {
    #[serde(rename = "buttonField", skip_serializing_if = "Option::is_none")]
    r#button_field: Option<String>,
    #[serde(rename = "loginUrlRegex", skip_serializing_if = "Option::is_none")]
    r#login_url_regex: Option<String>,
    #[serde(rename = "passwordField", skip_serializing_if = "Option::is_none")]
    r#password_field: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    r#url: Option<String>,
    #[serde(rename = "usernameField", skip_serializing_if = "Option::is_none")]
    r#username_field: Option<String>,
}

impl r#SwaApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#button_field: None,
          r#login_url_regex: None,
          r#password_field: None,
          r#url: None,
          r#username_field: None,
        }
    }

    pub fn set_button_field(&mut self, r#button_field: String) {
        self.r#button_field = Some(r#button_field);
    }

    pub fn with_button_field(mut self, r#button_field: String) -> Self {
        self.r#button_field = Some(r#button_field);
        self
    }

    pub fn r#button_field(&self) -> Option<&str> {
        self.r#button_field.as_ref().map(|x| x.borrow())
    }

    pub fn reset_button_field(&mut self) {
        self.r#button_field = None;
    }

    pub fn set_login_url_regex(&mut self, r#login_url_regex: String) {
        self.r#login_url_regex = Some(r#login_url_regex);
    }

    pub fn with_login_url_regex(mut self, r#login_url_regex: String) -> Self {
        self.r#login_url_regex = Some(r#login_url_regex);
        self
    }

    pub fn r#login_url_regex(&self) -> Option<&str> {
        self.r#login_url_regex.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login_url_regex(&mut self) {
        self.r#login_url_regex = None;
    }

    pub fn set_password_field(&mut self, r#password_field: String) {
        self.r#password_field = Some(r#password_field);
    }

    pub fn with_password_field(mut self, r#password_field: String) -> Self {
        self.r#password_field = Some(r#password_field);
        self
    }

    pub fn r#password_field(&self) -> Option<&str> {
        self.r#password_field.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password_field(&mut self) {
        self.r#password_field = None;
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

    pub fn set_username_field(&mut self, r#username_field: String) {
        self.r#username_field = Some(r#username_field);
    }

    pub fn with_username_field(mut self, r#username_field: String) -> Self {
        self.r#username_field = Some(r#username_field);
        self
    }

    pub fn r#username_field(&self) -> Option<&str> {
        self.r#username_field.as_ref().map(|x| x.borrow())
    }

    pub fn reset_username_field(&mut self) {
        self.r#username_field = None;
    }
}
