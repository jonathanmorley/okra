#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupProfile {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    r#description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
}

impl r#GroupProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#description: None,
          r#name: None,
        }
    }

    pub fn set_description(&mut self, r#description: String) {
        self.r#description = Some(r#description);
    }

    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = Some(r#description);
        self
    }

    pub fn r#description(&self) -> Option<&str> {
        self.r#description.as_ref().map(|x| x.borrow())
    }

    pub fn reset_description(&mut self) {
        self.r#description = None;
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
}
