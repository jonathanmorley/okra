#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ChangePasswordRequest {
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    r#new_password: Option<PasswordCredential>,
    #[serde(rename = "oldPassword", skip_serializing_if = "Option::is_none")]
    r#old_password: Option<PasswordCredential>,
}

impl r#ChangePasswordRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#new_password: None,
          r#old_password: None,
        }
    }

    pub fn set_new_password(&mut self, r#new_password: PasswordCredential) {
        self.r#new_password = Some(r#new_password);
    }

    pub fn with_new_password(mut self, r#new_password: PasswordCredential) -> Self {
        self.r#new_password = Some(r#new_password);
        self
    }

    pub fn r#new_password(&self) -> Option<&PasswordCredential> {
        self.r#new_password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_new_password(&mut self) {
        self.r#new_password = None;
    }

    pub fn set_old_password(&mut self, r#old_password: PasswordCredential) {
        self.r#old_password = Some(r#old_password);
    }

    pub fn with_old_password(mut self, r#old_password: PasswordCredential) -> Self {
        self.r#old_password = Some(r#old_password);
        self
    }

    pub fn r#old_password(&self) -> Option<&PasswordCredential> {
        self.r#old_password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_old_password(&mut self) {
        self.r#old_password = None;
    }
}
