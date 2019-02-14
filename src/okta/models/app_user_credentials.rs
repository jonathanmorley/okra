#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AppUserCredentials {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    r#password: Option<AppUserPasswordCredential>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    r#user_name: Option<String>,
}

impl r#AppUserCredentials {
    pub fn new(
    ) -> Self {
        Self {
          r#password: None,
          r#user_name: None,
        }
    }

    pub fn set_password(&mut self, r#password: AppUserPasswordCredential) {
        self.r#password = Some(r#password);
    }

    pub fn with_password(mut self, r#password: AppUserPasswordCredential) -> Self {
        self.r#password = Some(r#password);
        self
    }

    pub fn r#password(&self) -> Option<&AppUserPasswordCredential> {
        self.r#password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password(&mut self) {
        self.r#password = None;
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
