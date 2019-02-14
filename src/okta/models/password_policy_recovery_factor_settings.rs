#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryFactorSettings {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<String>,
}

impl r#PasswordPolicyRecoveryFactorSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#status: None,
        }
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
