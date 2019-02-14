#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyPasswordSettings {
    #[serde(rename = "age", skip_serializing_if = "Option::is_none")]
    r#age: Option<PasswordPolicyPasswordSettingsAge>,
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    r#complexity: Option<PasswordPolicyPasswordSettingsComplexity>,
    #[serde(rename = "lockout", skip_serializing_if = "Option::is_none")]
    r#lockout: Option<PasswordPolicyPasswordSettingsLockout>,
}

impl r#PasswordPolicyPasswordSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#age: None,
          r#complexity: None,
          r#lockout: None,
        }
    }

    pub fn set_age(&mut self, r#age: PasswordPolicyPasswordSettingsAge) {
        self.r#age = Some(r#age);
    }

    pub fn with_age(mut self, r#age: PasswordPolicyPasswordSettingsAge) -> Self {
        self.r#age = Some(r#age);
        self
    }

    pub fn r#age(&self) -> Option<&PasswordPolicyPasswordSettingsAge> {
        self.r#age.as_ref().map(|x| x.borrow())
    }

    pub fn reset_age(&mut self) {
        self.r#age = None;
    }

    pub fn set_complexity(&mut self, r#complexity: PasswordPolicyPasswordSettingsComplexity) {
        self.r#complexity = Some(r#complexity);
    }

    pub fn with_complexity(mut self, r#complexity: PasswordPolicyPasswordSettingsComplexity) -> Self {
        self.r#complexity = Some(r#complexity);
        self
    }

    pub fn r#complexity(&self) -> Option<&PasswordPolicyPasswordSettingsComplexity> {
        self.r#complexity.as_ref().map(|x| x.borrow())
    }

    pub fn reset_complexity(&mut self) {
        self.r#complexity = None;
    }

    pub fn set_lockout(&mut self, r#lockout: PasswordPolicyPasswordSettingsLockout) {
        self.r#lockout = Some(r#lockout);
    }

    pub fn with_lockout(mut self, r#lockout: PasswordPolicyPasswordSettingsLockout) -> Self {
        self.r#lockout = Some(r#lockout);
        self
    }

    pub fn r#lockout(&self) -> Option<&PasswordPolicyPasswordSettingsLockout> {
        self.r#lockout.as_ref().map(|x| x.borrow())
    }

    pub fn reset_lockout(&mut self) {
        self.r#lockout = None;
    }
}
