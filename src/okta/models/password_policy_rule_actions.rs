#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRuleActions {
    #[serde(rename = "passwordChange", skip_serializing_if = "Option::is_none")]
    r#password_change: Option<PasswordPolicyRuleAction>,
    #[serde(rename = "selfServicePasswordReset", skip_serializing_if = "Option::is_none")]
    r#self_service_password_reset: Option<PasswordPolicyRuleAction>,
    #[serde(rename = "selfServiceUnlock", skip_serializing_if = "Option::is_none")]
    r#self_service_unlock: Option<PasswordPolicyRuleAction>,
}

impl r#PasswordPolicyRuleActions {
    pub fn new(
    ) -> Self {
        Self {
          r#password_change: None,
          r#self_service_password_reset: None,
          r#self_service_unlock: None,
        }
    }

    pub fn set_password_change(&mut self, r#password_change: PasswordPolicyRuleAction) {
        self.r#password_change = Some(r#password_change);
    }

    pub fn with_password_change(mut self, r#password_change: PasswordPolicyRuleAction) -> Self {
        self.r#password_change = Some(r#password_change);
        self
    }

    pub fn r#password_change(&self) -> Option<&PasswordPolicyRuleAction> {
        self.r#password_change.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password_change(&mut self) {
        self.r#password_change = None;
    }

    pub fn set_self_service_password_reset(&mut self, r#self_service_password_reset: PasswordPolicyRuleAction) {
        self.r#self_service_password_reset = Some(r#self_service_password_reset);
    }

    pub fn with_self_service_password_reset(mut self, r#self_service_password_reset: PasswordPolicyRuleAction) -> Self {
        self.r#self_service_password_reset = Some(r#self_service_password_reset);
        self
    }

    pub fn r#self_service_password_reset(&self) -> Option<&PasswordPolicyRuleAction> {
        self.r#self_service_password_reset.as_ref().map(|x| x.borrow())
    }

    pub fn reset_self_service_password_reset(&mut self) {
        self.r#self_service_password_reset = None;
    }

    pub fn set_self_service_unlock(&mut self, r#self_service_unlock: PasswordPolicyRuleAction) {
        self.r#self_service_unlock = Some(r#self_service_unlock);
    }

    pub fn with_self_service_unlock(mut self, r#self_service_unlock: PasswordPolicyRuleAction) -> Self {
        self.r#self_service_unlock = Some(r#self_service_unlock);
        self
    }

    pub fn r#self_service_unlock(&self) -> Option<&PasswordPolicyRuleAction> {
        self.r#self_service_unlock.as_ref().map(|x| x.borrow())
    }

    pub fn reset_self_service_unlock(&mut self) {
        self.r#self_service_unlock = None;
    }
}
