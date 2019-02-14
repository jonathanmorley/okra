#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#TempPassword {
    #[serde(rename = "tempPassword", skip_serializing_if = "Option::is_none")]
    r#temp_password: Option<String>,
}

impl r#TempPassword {
    pub fn new(
    ) -> Self {
        Self {
          r#temp_password: None,
        }
    }

    pub fn set_temp_password(&mut self, r#temp_password: String) {
        self.r#temp_password = Some(r#temp_password);
    }

    pub fn with_temp_password(mut self, r#temp_password: String) -> Self {
        self.r#temp_password = Some(r#temp_password);
        self
    }

    pub fn r#temp_password(&self) -> Option<&str> {
        self.r#temp_password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_temp_password(&mut self) {
        self.r#temp_password = None;
    }
}
