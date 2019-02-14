#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicy {
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    r#conditions: Option<PasswordPolicyConditions>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    r#settings: Option<PasswordPolicySettings>,
}

impl r#PasswordPolicy {
    pub fn new(
    ) -> Self {
        Self {
          r#conditions: None,
          r#settings: None,
        }
    }

    pub fn set_conditions(&mut self, r#conditions: PasswordPolicyConditions) {
        self.r#conditions = Some(r#conditions);
    }

    pub fn with_conditions(mut self, r#conditions: PasswordPolicyConditions) -> Self {
        self.r#conditions = Some(r#conditions);
        self
    }

    pub fn r#conditions(&self) -> Option<&PasswordPolicyConditions> {
        self.r#conditions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_conditions(&mut self) {
        self.r#conditions = None;
    }

    pub fn set_settings(&mut self, r#settings: PasswordPolicySettings) {
        self.r#settings = Some(r#settings);
    }

    pub fn with_settings(mut self, r#settings: PasswordPolicySettings) -> Self {
        self.r#settings = Some(r#settings);
        self
    }

    pub fn r#settings(&self) -> Option<&PasswordPolicySettings> {
        self.r#settings.as_ref().map(|x| x.borrow())
    }

    pub fn reset_settings(&mut self) {
        self.r#settings = None;
    }
}
