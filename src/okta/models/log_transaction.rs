#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogTransaction {
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    r#detail: Option<Value>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#LogTransaction {
    pub fn new(
    ) -> Self {
        Self {
          r#detail: None,
          r#id: None,
          r#type: None,
        }
    }

    pub fn set_detail(&mut self, r#detail: Value) {
        self.r#detail = Some(r#detail);
    }

    pub fn with_detail(mut self, r#detail: Value) -> Self {
        self.r#detail = Some(r#detail);
        self
    }

    pub fn r#detail(&self) -> Option<&Value> {
        self.r#detail.as_ref().map(|x| x.borrow())
    }

    pub fn reset_detail(&mut self) {
        self.r#detail = None;
    }

    pub fn set_id(&mut self, r#id: String) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: String) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn r#id(&self) -> Option<&str> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
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
