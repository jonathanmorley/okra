#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoverySettings {
    #[serde(rename = "factors", skip_serializing_if = "Option::is_none")]
    r#factors: Option<PasswordPolicyRecoveryFactors>,
}

impl r#PasswordPolicyRecoverySettings {
    pub fn new(
    ) -> Self {
        Self {
          r#factors: None,
        }
    }

    pub fn set_factors(&mut self, r#factors: PasswordPolicyRecoveryFactors) {
        self.r#factors = Some(r#factors);
    }

    pub fn with_factors(mut self, r#factors: PasswordPolicyRecoveryFactors) -> Self {
        self.r#factors = Some(r#factors);
        self
    }

    pub fn r#factors(&self) -> Option<&PasswordPolicyRecoveryFactors> {
        self.r#factors.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factors(&mut self) {
        self.r#factors = None;
    }
}
