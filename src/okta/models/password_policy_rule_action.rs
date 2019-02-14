#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRuleAction {
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    r#access: Option<String>,
}

impl r#PasswordPolicyRuleAction {
    pub fn new(
    ) -> Self {
        Self {
          r#access: None,
        }
    }

    pub fn set_access(&mut self, r#access: String) {
        self.r#access = Some(r#access);
    }

    pub fn with_access(mut self, r#access: String) -> Self {
        self.r#access = Some(r#access);
        self
    }

    pub fn r#access(&self) -> Option<&str> {
        self.r#access.as_ref().map(|x| x.borrow())
    }

    pub fn reset_access(&mut self) {
        self.r#access = None;
    }
}
