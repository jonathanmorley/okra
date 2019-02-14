#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryEmailProperties {
    #[serde(rename = "recoveryToken", skip_serializing_if = "Option::is_none")]
    r#recovery_token: Option<PasswordPolicyRecoveryEmailRecoveryToken>,
}

impl r#PasswordPolicyRecoveryEmailProperties {
    pub fn new(
    ) -> Self {
        Self {
          r#recovery_token: None,
        }
    }

    pub fn set_recovery_token(&mut self, r#recovery_token: PasswordPolicyRecoveryEmailRecoveryToken) {
        self.r#recovery_token = Some(r#recovery_token);
    }

    pub fn with_recovery_token(mut self, r#recovery_token: PasswordPolicyRecoveryEmailRecoveryToken) -> Self {
        self.r#recovery_token = Some(r#recovery_token);
        self
    }

    pub fn r#recovery_token(&self) -> Option<&PasswordPolicyRecoveryEmailRecoveryToken> {
        self.r#recovery_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recovery_token(&mut self) {
        self.r#recovery_token = None;
    }
}
