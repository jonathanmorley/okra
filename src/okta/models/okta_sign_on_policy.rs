#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicy {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    r#conditions: Option<OktaSignOnPolicyConditions>,
}

impl r#OktaSignOnPolicy {
    pub fn new(
    ) -> Self {
        Self {
          r#conditions: None,
        }
    }

    pub fn set_conditions(&mut self, r#conditions: OktaSignOnPolicyConditions) {
        self.r#conditions = Some(r#conditions);
    }

    pub fn with_conditions(mut self, r#conditions: OktaSignOnPolicyConditions) -> Self {
        self.r#conditions = Some(r#conditions);
        self
    }

    pub fn r#conditions(&self) -> Option<&OktaSignOnPolicyConditions> {
        self.r#conditions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_conditions(&mut self) {
        self.r#conditions = None;
    }
}
