#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SwaThreeFieldApplicationSettingsApplication {
    #[serde(rename = "buttonSelector", skip_serializing_if = "Option::is_none")]
    r#button_selector: Option<String>,
    #[serde(rename = "extraFieldSelector", skip_serializing_if = "Option::is_none")]
    r#extra_field_selector: Option<String>,
    #[serde(rename = "extraFieldValue", skip_serializing_if = "Option::is_none")]
    r#extra_field_value: Option<String>,
    #[serde(rename = "loginUrlRegex", skip_serializing_if = "Option::is_none")]
    r#login_url_regex: Option<String>,
    #[serde(rename = "passwordSelector", skip_serializing_if = "Option::is_none")]
    r#password_selector: Option<String>,
    #[serde(rename = "targetUrl", skip_serializing_if = "Option::is_none")]
    r#target_url: Option<String>,
    #[serde(rename = "userNameSelector", skip_serializing_if = "Option::is_none")]
    r#user_name_selector: Option<String>,
}

impl r#SwaThreeFieldApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#button_selector: None,
          r#extra_field_selector: None,
          r#extra_field_value: None,
          r#login_url_regex: None,
          r#password_selector: None,
          r#target_url: None,
          r#user_name_selector: None,
        }
    }

    pub fn set_button_selector(&mut self, r#button_selector: String) {
        self.r#button_selector = Some(r#button_selector);
    }

    pub fn with_button_selector(mut self, r#button_selector: String) -> Self {
        self.r#button_selector = Some(r#button_selector);
        self
    }

    pub fn r#button_selector(&self) -> Option<&str> {
        self.r#button_selector.as_ref().map(|x| x.borrow())
    }

    pub fn reset_button_selector(&mut self) {
        self.r#button_selector = None;
    }

    pub fn set_extra_field_selector(&mut self, r#extra_field_selector: String) {
        self.r#extra_field_selector = Some(r#extra_field_selector);
    }

    pub fn with_extra_field_selector(mut self, r#extra_field_selector: String) -> Self {
        self.r#extra_field_selector = Some(r#extra_field_selector);
        self
    }

    pub fn r#extra_field_selector(&self) -> Option<&str> {
        self.r#extra_field_selector.as_ref().map(|x| x.borrow())
    }

    pub fn reset_extra_field_selector(&mut self) {
        self.r#extra_field_selector = None;
    }

    pub fn set_extra_field_value(&mut self, r#extra_field_value: String) {
        self.r#extra_field_value = Some(r#extra_field_value);
    }

    pub fn with_extra_field_value(mut self, r#extra_field_value: String) -> Self {
        self.r#extra_field_value = Some(r#extra_field_value);
        self
    }

    pub fn r#extra_field_value(&self) -> Option<&str> {
        self.r#extra_field_value.as_ref().map(|x| x.borrow())
    }

    pub fn reset_extra_field_value(&mut self) {
        self.r#extra_field_value = None;
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

    pub fn set_password_selector(&mut self, r#password_selector: String) {
        self.r#password_selector = Some(r#password_selector);
    }

    pub fn with_password_selector(mut self, r#password_selector: String) -> Self {
        self.r#password_selector = Some(r#password_selector);
        self
    }

    pub fn r#password_selector(&self) -> Option<&str> {
        self.r#password_selector.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password_selector(&mut self) {
        self.r#password_selector = None;
    }

    pub fn set_target_url(&mut self, r#target_url: String) {
        self.r#target_url = Some(r#target_url);
    }

    pub fn with_target_url(mut self, r#target_url: String) -> Self {
        self.r#target_url = Some(r#target_url);
        self
    }

    pub fn r#target_url(&self) -> Option<&str> {
        self.r#target_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_target_url(&mut self) {
        self.r#target_url = None;
    }

    pub fn set_user_name_selector(&mut self, r#user_name_selector: String) {
        self.r#user_name_selector = Some(r#user_name_selector);
    }

    pub fn with_user_name_selector(mut self, r#user_name_selector: String) -> Self {
        self.r#user_name_selector = Some(r#user_name_selector);
        self
    }

    pub fn r#user_name_selector(&self) -> Option<&str> {
        self.r#user_name_selector.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_name_selector(&mut self) {
        self.r#user_name_selector = None;
    }
}
