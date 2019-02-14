#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#UserProfile {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    r#email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    r#first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    r#last_name: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    r#login: Option<String>,
    #[serde(rename = "mobilePhone", skip_serializing_if = "Option::is_none")]
    r#mobile_phone: Option<String>,
    #[serde(rename = "secondEmail", skip_serializing_if = "Option::is_none")]
    r#second_email: Option<String>,
}

impl r#UserProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#email: None,
          r#first_name: None,
          r#last_name: None,
          r#login: None,
          r#mobile_phone: None,
          r#second_email: None,
        }
    }

    pub fn set_email(&mut self, r#email: String) {
        self.r#email = Some(r#email);
    }

    pub fn with_email(mut self, r#email: String) -> Self {
        self.r#email = Some(r#email);
        self
    }

    pub fn r#email(&self) -> Option<&str> {
        self.r#email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_email(&mut self) {
        self.r#email = None;
    }

    pub fn set_first_name(&mut self, r#first_name: String) {
        self.r#first_name = Some(r#first_name);
    }

    pub fn with_first_name(mut self, r#first_name: String) -> Self {
        self.r#first_name = Some(r#first_name);
        self
    }

    pub fn r#first_name(&self) -> Option<&str> {
        self.r#first_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_first_name(&mut self) {
        self.r#first_name = None;
    }

    pub fn set_last_name(&mut self, r#last_name: String) {
        self.r#last_name = Some(r#last_name);
    }

    pub fn with_last_name(mut self, r#last_name: String) -> Self {
        self.r#last_name = Some(r#last_name);
        self
    }

    pub fn r#last_name(&self) -> Option<&str> {
        self.r#last_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_name(&mut self) {
        self.r#last_name = None;
    }

    pub fn set_login(&mut self, r#login: String) {
        self.r#login = Some(r#login);
    }

    pub fn with_login(mut self, r#login: String) -> Self {
        self.r#login = Some(r#login);
        self
    }

    pub fn r#login(&self) -> Option<&str> {
        self.r#login.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login(&mut self) {
        self.r#login = None;
    }

    pub fn set_mobile_phone(&mut self, r#mobile_phone: String) {
        self.r#mobile_phone = Some(r#mobile_phone);
    }

    pub fn with_mobile_phone(mut self, r#mobile_phone: String) -> Self {
        self.r#mobile_phone = Some(r#mobile_phone);
        self
    }

    pub fn r#mobile_phone(&self) -> Option<&str> {
        self.r#mobile_phone.as_ref().map(|x| x.borrow())
    }

    pub fn reset_mobile_phone(&mut self) {
        self.r#mobile_phone = None;
    }

    pub fn set_second_email(&mut self, r#second_email: String) {
        self.r#second_email = Some(r#second_email);
    }

    pub fn with_second_email(mut self, r#second_email: String) -> Self {
        self.r#second_email = Some(r#second_email);
        self
    }

    pub fn r#second_email(&self) -> Option<&str> {
        self.r#second_email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_second_email(&mut self) {
        self.r#second_email = None;
    }
}
