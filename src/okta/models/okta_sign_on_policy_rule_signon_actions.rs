#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyRuleSignonActions {
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    r#access: Option<String>,
    #[serde(rename = "factorLifetime", skip_serializing_if = "Option::is_none")]
    r#factor_lifetime: Option<i32>,
    #[serde(rename = "factorPromptMode", skip_serializing_if = "Option::is_none")]
    r#factor_prompt_mode: Option<String>,
    #[serde(rename = "rememberDeviceByDefault", skip_serializing_if = "Option::is_none")]
    r#remember_device_by_default: Option<bool>,
    #[serde(rename = "requireFactor", skip_serializing_if = "Option::is_none")]
    r#require_factor: Option<bool>,
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    r#session: Option<OktaSignOnPolicyRuleSignonSessionActions>,
}

impl r#OktaSignOnPolicyRuleSignonActions {
    pub fn new(
    ) -> Self {
        Self {
          r#access: None,
          r#factor_lifetime: None,
          r#factor_prompt_mode: None,
          r#remember_device_by_default: None,
          r#require_factor: None,
          r#session: None,
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

    pub fn set_factor_lifetime(&mut self, r#factor_lifetime: i32) {
        self.r#factor_lifetime = Some(r#factor_lifetime);
    }

    pub fn with_factor_lifetime(mut self, r#factor_lifetime: i32) -> Self {
        self.r#factor_lifetime = Some(r#factor_lifetime);
        self
    }

    pub fn r#factor_lifetime(&self) -> Option<&i32> {
        self.r#factor_lifetime.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factor_lifetime(&mut self) {
        self.r#factor_lifetime = None;
    }

    pub fn set_factor_prompt_mode(&mut self, r#factor_prompt_mode: String) {
        self.r#factor_prompt_mode = Some(r#factor_prompt_mode);
    }

    pub fn with_factor_prompt_mode(mut self, r#factor_prompt_mode: String) -> Self {
        self.r#factor_prompt_mode = Some(r#factor_prompt_mode);
        self
    }

    pub fn r#factor_prompt_mode(&self) -> Option<&str> {
        self.r#factor_prompt_mode.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factor_prompt_mode(&mut self) {
        self.r#factor_prompt_mode = None;
    }

    pub fn set_remember_device_by_default(&mut self, r#remember_device_by_default: bool) {
        self.r#remember_device_by_default = Some(r#remember_device_by_default);
    }

    pub fn with_remember_device_by_default(mut self, r#remember_device_by_default: bool) -> Self {
        self.r#remember_device_by_default = Some(r#remember_device_by_default);
        self
    }

    pub fn r#remember_device_by_default(&self) -> Option<&bool> {
        self.r#remember_device_by_default.as_ref().map(|x| x.borrow())
    }

    pub fn reset_remember_device_by_default(&mut self) {
        self.r#remember_device_by_default = None;
    }

    pub fn set_require_factor(&mut self, r#require_factor: bool) {
        self.r#require_factor = Some(r#require_factor);
    }

    pub fn with_require_factor(mut self, r#require_factor: bool) -> Self {
        self.r#require_factor = Some(r#require_factor);
        self
    }

    pub fn r#require_factor(&self) -> Option<&bool> {
        self.r#require_factor.as_ref().map(|x| x.borrow())
    }

    pub fn reset_require_factor(&mut self) {
        self.r#require_factor = None;
    }

    pub fn set_session(&mut self, r#session: OktaSignOnPolicyRuleSignonSessionActions) {
        self.r#session = Some(r#session);
    }

    pub fn with_session(mut self, r#session: OktaSignOnPolicyRuleSignonSessionActions) -> Self {
        self.r#session = Some(r#session);
        self
    }

    pub fn r#session(&self) -> Option<&OktaSignOnPolicyRuleSignonSessionActions> {
        self.r#session.as_ref().map(|x| x.borrow())
    }

    pub fn reset_session(&mut self) {
        self.r#session = None;
    }
}
