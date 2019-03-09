#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyDelegationSettings {
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    r#options: Option<PasswordPolicyDelegationSettingsOptions>,
}

impl r#PasswordPolicyDelegationSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#options: None,
        }
    }

    pub fn set_options(&mut self, r#options: PasswordPolicyDelegationSettingsOptions) {
        self.r#options = Some(r#options);
    }

    pub fn with_options(mut self, r#options: PasswordPolicyDelegationSettingsOptions) -> Self {
        self.r#options = Some(r#options);
        self
    }

    pub fn r#options(&self) -> Option<&PasswordPolicyDelegationSettingsOptions> {
        self.r#options.as_ref().map(|x| x.borrow())
    }

    pub fn reset_options(&mut self) {
        self.r#options = None;
    }
}
