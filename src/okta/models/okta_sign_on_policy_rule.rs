#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyRule {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    r#actions: Option<OktaSignOnPolicyRuleActions>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    r#conditions: Option<OktaSignOnPolicyRuleConditions>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
}

impl r#OktaSignOnPolicyRule {
    pub fn new(
    ) -> Self {
        Self {
          r#actions: None,
          r#conditions: None,
          r#name: None,
        }
    }

    pub fn set_actions(&mut self, r#actions: OktaSignOnPolicyRuleActions) {
        self.r#actions = Some(r#actions);
    }

    pub fn with_actions(mut self, r#actions: OktaSignOnPolicyRuleActions) -> Self {
        self.r#actions = Some(r#actions);
        self
    }

    pub fn r#actions(&self) -> Option<&OktaSignOnPolicyRuleActions> {
        self.r#actions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_actions(&mut self) {
        self.r#actions = None;
    }

    pub fn set_conditions(&mut self, r#conditions: OktaSignOnPolicyRuleConditions) {
        self.r#conditions = Some(r#conditions);
    }

    pub fn with_conditions(mut self, r#conditions: OktaSignOnPolicyRuleConditions) -> Self {
        self.r#conditions = Some(r#conditions);
        self
    }

    pub fn r#conditions(&self) -> Option<&OktaSignOnPolicyRuleConditions> {
        self.r#conditions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_conditions(&mut self) {
        self.r#conditions = None;
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = Some(r#name);
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = Some(r#name);
        self
    }

    pub fn r#name(&self) -> Option<&str> {
        self.r#name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name(&mut self) {
        self.r#name = None;
    }
}
