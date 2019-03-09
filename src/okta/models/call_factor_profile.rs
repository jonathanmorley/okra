#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#CallFactorProfile {
    #[serde(rename = "phoneExtension", skip_serializing_if = "Option::is_none")]
    r#phone_extension: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    r#phone_number: Option<String>,
}

impl r#CallFactorProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#phone_extension: None,
          r#phone_number: None,
        }
    }

    pub fn set_phone_extension(&mut self, r#phone_extension: String) {
        self.r#phone_extension = Some(r#phone_extension);
    }

    pub fn with_phone_extension(mut self, r#phone_extension: String) -> Self {
        self.r#phone_extension = Some(r#phone_extension);
        self
    }

    pub fn r#phone_extension(&self) -> Option<&str> {
        self.r#phone_extension.as_ref().map(|x| x.borrow())
    }

    pub fn reset_phone_extension(&mut self) {
        self.r#phone_extension = None;
    }

    pub fn set_phone_number(&mut self, r#phone_number: String) {
        self.r#phone_number = Some(r#phone_number);
    }

    pub fn with_phone_number(mut self, r#phone_number: String) -> Self {
        self.r#phone_number = Some(r#phone_number);
        self
    }

    pub fn r#phone_number(&self) -> Option<&str> {
        self.r#phone_number.as_ref().map(|x| x.borrow())
    }

    pub fn reset_phone_number(&mut self) {
        self.r#phone_number = None;
    }
}
