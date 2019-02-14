#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryEmailRecoveryToken {
    #[serde(rename = "tokenLifetimeMinutes", skip_serializing_if = "Option::is_none")]
    r#token_lifetime_minutes: Option<i32>,
}

impl r#PasswordPolicyRecoveryEmailRecoveryToken {
    pub fn new(
    ) -> Self {
        Self {
          r#token_lifetime_minutes: None,
        }
    }

    pub fn set_token_lifetime_minutes(&mut self, r#token_lifetime_minutes: i32) {
        self.r#token_lifetime_minutes = Some(r#token_lifetime_minutes);
    }

    pub fn with_token_lifetime_minutes(mut self, r#token_lifetime_minutes: i32) -> Self {
        self.r#token_lifetime_minutes = Some(r#token_lifetime_minutes);
        self
    }

    pub fn r#token_lifetime_minutes(&self) -> Option<&i32> {
        self.r#token_lifetime_minutes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_token_lifetime_minutes(&mut self) {
        self.r#token_lifetime_minutes = None;
    }
}
