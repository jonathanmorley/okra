#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationCredentialsUsernameTemplate {
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    r#suffix: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    r#template: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#ApplicationCredentialsUsernameTemplate {
    pub fn new(
    ) -> Self {
        Self {
          r#suffix: None,
          r#template: None,
          r#type: None,
        }
    }

    pub fn set_suffix(&mut self, r#suffix: String) {
        self.r#suffix = Some(r#suffix);
    }

    pub fn with_suffix(mut self, r#suffix: String) -> Self {
        self.r#suffix = Some(r#suffix);
        self
    }

    pub fn r#suffix(&self) -> Option<&str> {
        self.r#suffix.as_ref().map(|x| x.borrow())
    }

    pub fn reset_suffix(&mut self) {
        self.r#suffix = None;
    }

    pub fn set_template(&mut self, r#template: String) {
        self.r#template = Some(r#template);
    }

    pub fn with_template(mut self, r#template: String) -> Self {
        self.r#template = Some(r#template);
        self
    }

    pub fn r#template(&self) -> Option<&str> {
        self.r#template.as_ref().map(|x| x.borrow())
    }

    pub fn reset_template(&mut self) {
        self.r#template = None;
    }

    pub fn set_type(&mut self, r#type: String) {
        self.r#type = Some(r#type);
    }

    pub fn with_type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn r#type(&self) -> Option<&str> {
        self.r#type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_type(&mut self) {
        self.r#type = None;
    }
}
