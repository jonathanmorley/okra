#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogTarget {
    #[serde(rename = "alternateId", skip_serializing_if = "Option::is_none")]
    r#alternate_id: Option<String>,
    #[serde(rename = "detailEntry", skip_serializing_if = "Option::is_none")]
    r#detail_entry: Option<Value>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    r#display_name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#LogTarget {
    pub fn new(
    ) -> Self {
        Self {
          r#alternate_id: None,
          r#detail_entry: None,
          r#display_name: None,
          r#id: None,
          r#type: None,
        }
    }

    pub fn set_alternate_id(&mut self, r#alternate_id: String) {
        self.r#alternate_id = Some(r#alternate_id);
    }

    pub fn with_alternate_id(mut self, r#alternate_id: String) -> Self {
        self.r#alternate_id = Some(r#alternate_id);
        self
    }

    pub fn r#alternate_id(&self) -> Option<&str> {
        self.r#alternate_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_alternate_id(&mut self) {
        self.r#alternate_id = None;
    }

    pub fn set_detail_entry(&mut self, r#detail_entry: Value) {
        self.r#detail_entry = Some(r#detail_entry);
    }

    pub fn with_detail_entry(mut self, r#detail_entry: Value) -> Self {
        self.r#detail_entry = Some(r#detail_entry);
        self
    }

    pub fn r#detail_entry(&self) -> Option<&Value> {
        self.r#detail_entry.as_ref().map(|x| x.borrow())
    }

    pub fn reset_detail_entry(&mut self) {
        self.r#detail_entry = None;
    }

    pub fn set_display_name(&mut self, r#display_name: String) {
        self.r#display_name = Some(r#display_name);
    }

    pub fn with_display_name(mut self, r#display_name: String) -> Self {
        self.r#display_name = Some(r#display_name);
        self
    }

    pub fn r#display_name(&self) -> Option<&str> {
        self.r#display_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_display_name(&mut self) {
        self.r#display_name = None;
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
