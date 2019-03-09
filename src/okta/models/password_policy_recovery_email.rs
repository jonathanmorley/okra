#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryEmail {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    r#properties: Option<PasswordPolicyRecoveryEmailProperties>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<String>,
}

impl r#PasswordPolicyRecoveryEmail {
    pub fn new(
    ) -> Self {
        Self {
          r#properties: None,
          r#status: None,
        }
    }

    pub fn set_properties(&mut self, r#properties: PasswordPolicyRecoveryEmailProperties) {
        self.r#properties = Some(r#properties);
    }

    pub fn with_properties(mut self, r#properties: PasswordPolicyRecoveryEmailProperties) -> Self {
        self.r#properties = Some(r#properties);
        self
    }

    pub fn r#properties(&self) -> Option<&PasswordPolicyRecoveryEmailProperties> {
        self.r#properties.as_ref().map(|x| x.borrow())
    }

    pub fn reset_properties(&mut self) {
        self.r#properties = None;
    }

    pub fn set_status(&mut self, r#status: String) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: String) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&str> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }
}
