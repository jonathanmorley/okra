#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PolicyRuleAuthContextCondition {
    #[serde(rename = "authType", skip_serializing_if = "Option::is_none")]
    r#auth_type: Option<String>,
}

impl r#PolicyRuleAuthContextCondition {
    pub fn new(
    ) -> Self {
        Self {
          r#auth_type: None,
        }
    }

    pub fn set_auth_type(&mut self, r#auth_type: String) {
        self.r#auth_type = Some(r#auth_type);
    }

    pub fn with_auth_type(mut self, r#auth_type: String) -> Self {
        self.r#auth_type = Some(r#auth_type);
        self
    }

    pub fn r#auth_type(&self) -> Option<&str> {
        self.r#auth_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auth_type(&mut self) {
        self.r#auth_type = None;
    }
}
