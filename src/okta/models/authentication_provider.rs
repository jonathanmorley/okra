#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AuthenticationProvider {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<AuthenticationProviderType>,
}

impl r#AuthenticationProvider {
    pub fn new(
    ) -> Self {
        Self {
          r#name: None,
          r#type: None,
        }
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = Some(r#name);
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = Some(r#name);
        self
    }

    pub fn r#name(&self) -> Option<&str> {
        self.r#name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name(&mut self) {
        self.r#name = None;
    }

    pub fn set_type(&mut self, r#type: AuthenticationProviderType) {
        self.r#type = Some(r#type);
    }

    pub fn with_type(mut self, r#type: AuthenticationProviderType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn r#type(&self) -> Option<&AuthenticationProviderType> {
        self.r#type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_type(&mut self) {
        self.r#type = None;
    }
}
