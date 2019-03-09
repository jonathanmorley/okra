#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SchemeApplicationCredentials {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    r#password: Option<PasswordCredential>,
    #[serde(rename = "revealPassword", skip_serializing_if = "Option::is_none")]
    r#reveal_password: Option<bool>,
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    r#scheme: Option<ApplicationCredentialsScheme>,
    #[serde(rename = "signing", skip_serializing_if = "Option::is_none")]
    r#signing: Option<ApplicationCredentialsSigning>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    r#user_name: Option<String>,
}

impl r#SchemeApplicationCredentials {
    pub fn new(
    ) -> Self {
        Self {
          r#password: None,
          r#reveal_password: None,
          r#scheme: None,
          r#signing: None,
          r#user_name: None,
        }
    }

    pub fn set_password(&mut self, r#password: PasswordCredential) {
        self.r#password = Some(r#password);
    }

    pub fn with_password(mut self, r#password: PasswordCredential) -> Self {
        self.r#password = Some(r#password);
        self
    }

    pub fn r#password(&self) -> Option<&PasswordCredential> {
        self.r#password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password(&mut self) {
        self.r#password = None;
    }

    pub fn set_reveal_password(&mut self, r#reveal_password: bool) {
        self.r#reveal_password = Some(r#reveal_password);
    }

    pub fn with_reveal_password(mut self, r#reveal_password: bool) -> Self {
        self.r#reveal_password = Some(r#reveal_password);
        self
    }

    pub fn r#reveal_password(&self) -> Option<&bool> {
        self.r#reveal_password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_reveal_password(&mut self) {
        self.r#reveal_password = None;
    }

    pub fn set_scheme(&mut self, r#scheme: ApplicationCredentialsScheme) {
        self.r#scheme = Some(r#scheme);
    }

    pub fn with_scheme(mut self, r#scheme: ApplicationCredentialsScheme) -> Self {
        self.r#scheme = Some(r#scheme);
        self
    }

    pub fn r#scheme(&self) -> Option<&ApplicationCredentialsScheme> {
        self.r#scheme.as_ref().map(|x| x.borrow())
    }

    pub fn reset_scheme(&mut self) {
        self.r#scheme = None;
    }

    pub fn set_signing(&mut self, r#signing: ApplicationCredentialsSigning) {
        self.r#signing = Some(r#signing);
    }

    pub fn with_signing(mut self, r#signing: ApplicationCredentialsSigning) -> Self {
        self.r#signing = Some(r#signing);
        self
    }

    pub fn r#signing(&self) -> Option<&ApplicationCredentialsSigning> {
        self.r#signing.as_ref().map(|x| x.borrow())
    }

    pub fn reset_signing(&mut self) {
        self.r#signing = None;
    }

    pub fn set_user_name(&mut self, r#user_name: String) {
        self.r#user_name = Some(r#user_name);
    }

    pub fn with_user_name(mut self, r#user_name: String) -> Self {
        self.r#user_name = Some(r#user_name);
        self
    }

    pub fn r#user_name(&self) -> Option<&str> {
        self.r#user_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_name(&mut self) {
        self.r#user_name = None;
    }
}
