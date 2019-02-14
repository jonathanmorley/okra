#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SecurePasswordStoreApplicationSettingsApplication {
    #[serde(rename = "optionalField1", skip_serializing_if = "Option::is_none")]
    r#optional_field1: Option<String>,
    #[serde(rename = "optionalField1Value", skip_serializing_if = "Option::is_none")]
    r#optional_field1_value: Option<String>,
    #[serde(rename = "optionalField2", skip_serializing_if = "Option::is_none")]
    r#optional_field2: Option<String>,
    #[serde(rename = "optionalField2Value", skip_serializing_if = "Option::is_none")]
    r#optional_field2_value: Option<String>,
    #[serde(rename = "optionalField3", skip_serializing_if = "Option::is_none")]
    r#optional_field3: Option<String>,
    #[serde(rename = "optionalField3Value", skip_serializing_if = "Option::is_none")]
    r#optional_field3_value: Option<String>,
    #[serde(rename = "passwordField", skip_serializing_if = "Option::is_none")]
    r#password_field: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    r#url: Option<String>,
    #[serde(rename = "usernameField", skip_serializing_if = "Option::is_none")]
    r#username_field: Option<String>,
}

impl r#SecurePasswordStoreApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#optional_field1: None,
          r#optional_field1_value: None,
          r#optional_field2: None,
          r#optional_field2_value: None,
          r#optional_field3: None,
          r#optional_field3_value: None,
          r#password_field: None,
          r#url: None,
          r#username_field: None,
        }
    }

    pub fn set_optional_field1(&mut self, r#optional_field1: String) {
        self.r#optional_field1 = Some(r#optional_field1);
    }

    pub fn with_optional_field1(mut self, r#optional_field1: String) -> Self {
        self.r#optional_field1 = Some(r#optional_field1);
        self
    }

    pub fn r#optional_field1(&self) -> Option<&str> {
        self.r#optional_field1.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field1(&mut self) {
        self.r#optional_field1 = None;
    }

    pub fn set_optional_field1_value(&mut self, r#optional_field1_value: String) {
        self.r#optional_field1_value = Some(r#optional_field1_value);
    }

    pub fn with_optional_field1_value(mut self, r#optional_field1_value: String) -> Self {
        self.r#optional_field1_value = Some(r#optional_field1_value);
        self
    }

    pub fn r#optional_field1_value(&self) -> Option<&str> {
        self.r#optional_field1_value.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field1_value(&mut self) {
        self.r#optional_field1_value = None;
    }

    pub fn set_optional_field2(&mut self, r#optional_field2: String) {
        self.r#optional_field2 = Some(r#optional_field2);
    }

    pub fn with_optional_field2(mut self, r#optional_field2: String) -> Self {
        self.r#optional_field2 = Some(r#optional_field2);
        self
    }

    pub fn r#optional_field2(&self) -> Option<&str> {
        self.r#optional_field2.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field2(&mut self) {
        self.r#optional_field2 = None;
    }

    pub fn set_optional_field2_value(&mut self, r#optional_field2_value: String) {
        self.r#optional_field2_value = Some(r#optional_field2_value);
    }

    pub fn with_optional_field2_value(mut self, r#optional_field2_value: String) -> Self {
        self.r#optional_field2_value = Some(r#optional_field2_value);
        self
    }

    pub fn r#optional_field2_value(&self) -> Option<&str> {
        self.r#optional_field2_value.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field2_value(&mut self) {
        self.r#optional_field2_value = None;
    }

    pub fn set_optional_field3(&mut self, r#optional_field3: String) {
        self.r#optional_field3 = Some(r#optional_field3);
    }

    pub fn with_optional_field3(mut self, r#optional_field3: String) -> Self {
        self.r#optional_field3 = Some(r#optional_field3);
        self
    }

    pub fn r#optional_field3(&self) -> Option<&str> {
        self.r#optional_field3.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field3(&mut self) {
        self.r#optional_field3 = None;
    }

    pub fn set_optional_field3_value(&mut self, r#optional_field3_value: String) {
        self.r#optional_field3_value = Some(r#optional_field3_value);
    }

    pub fn with_optional_field3_value(mut self, r#optional_field3_value: String) -> Self {
        self.r#optional_field3_value = Some(r#optional_field3_value);
        self
    }

    pub fn r#optional_field3_value(&self) -> Option<&str> {
        self.r#optional_field3_value.as_ref().map(|x| x.borrow())
    }

    pub fn reset_optional_field3_value(&mut self) {
        self.r#optional_field3_value = None;
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
