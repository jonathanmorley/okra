#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyDelegationSettingsOptions {
    #[serde(rename = "skipUnlock", skip_serializing_if = "Option::is_none")]
    r#skip_unlock: Option<bool>,
}

impl r#PasswordPolicyDelegationSettingsOptions {
    pub fn new(
    ) -> Self {
        Self {
          r#skip_unlock: None,
        }
    }

    pub fn set_skip_unlock(&mut self, r#skip_unlock: bool) {
        self.r#skip_unlock = Some(r#skip_unlock);
    }

    pub fn with_skip_unlock(mut self, r#skip_unlock: bool) -> Self {
        self.r#skip_unlock = Some(r#skip_unlock);
        self
    }

    pub fn r#skip_unlock(&self) -> Option<&bool> {
        self.r#skip_unlock.as_ref().map(|x| x.borrow())
    }

    pub fn reset_skip_unlock(&mut self) {
        self.r#skip_unlock = None;
    }
}
