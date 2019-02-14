#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicySettings {
    #[serde(rename = "delegation", skip_serializing_if = "Option::is_none")]
    r#delegation: Option<PasswordPolicyDelegationSettings>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    r#password: Option<PasswordPolicyPasswordSettings>,
    #[serde(rename = "recovery", skip_serializing_if = "Option::is_none")]
    r#recovery: Option<PasswordPolicyRecoverySettings>,
}

impl r#PasswordPolicySettings {
    pub fn new(
    ) -> Self {
        Self {
          r#delegation: None,
          r#password: None,
          r#recovery: None,
        }
    }

    pub fn set_delegation(&mut self, r#delegation: PasswordPolicyDelegationSettings) {
        self.r#delegation = Some(r#delegation);
    }

    pub fn with_delegation(mut self, r#delegation: PasswordPolicyDelegationSettings) -> Self {
        self.r#delegation = Some(r#delegation);
        self
    }

    pub fn r#delegation(&self) -> Option<&PasswordPolicyDelegationSettings> {
        self.r#delegation.as_ref().map(|x| x.borrow())
    }

    pub fn reset_delegation(&mut self) {
        self.r#delegation = None;
    }

    pub fn set_password(&mut self, r#password: PasswordPolicyPasswordSettings) {
        self.r#password = Some(r#password);
    }

    pub fn with_password(mut self, r#password: PasswordPolicyPasswordSettings) -> Self {
        self.r#password = Some(r#password);
        self
    }

    pub fn r#password(&self) -> Option<&PasswordPolicyPasswordSettings> {
        self.r#password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password(&mut self) {
        self.r#password = None;
    }

    pub fn set_recovery(&mut self, r#recovery: PasswordPolicyRecoverySettings) {
        self.r#recovery = Some(r#recovery);
    }

    pub fn with_recovery(mut self, r#recovery: PasswordPolicyRecoverySettings) -> Self {
        self.r#recovery = Some(r#recovery);
        self
    }

    pub fn r#recovery(&self) -> Option<&PasswordPolicyRecoverySettings> {
        self.r#recovery.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recovery(&mut self) {
        self.r#recovery = None;
    }
}
