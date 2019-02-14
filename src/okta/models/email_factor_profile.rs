#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#EmailFactorProfile {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    r#email: Option<String>,
}

impl r#EmailFactorProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#email: None,
        }
    }

    pub fn set_email(&mut self, r#email: String) {
        self.r#email = Some(r#email);
    }

    pub fn with_email(mut self, r#email: String) -> Self {
        self.r#email = Some(r#email);
        self
    }

    pub fn r#email(&self) -> Option<&str> {
        self.r#email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_email(&mut self) {
        self.r#email = None;
    }
}
