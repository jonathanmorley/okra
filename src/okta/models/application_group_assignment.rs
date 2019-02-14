#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationGroupAssignment {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    r#priority: Option<i32>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<Value>,
}

impl r#ApplicationGroupAssignment {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#id: None,
          r#last_updated: None,
          r#priority: None,
          r#profile: None,
        }
    }

    pub fn set_embedded(&mut self, r#embedded: Value) {
        self.r#embedded = Some(r#embedded);
    }

    pub fn with_embedded(mut self, r#embedded: Value) -> Self {
        self.r#embedded = Some(r#embedded);
        self
    }

    pub fn r#embedded(&self) -> Option<&Value> {
        self.r#embedded.as_ref().map(|x| x.borrow())
    }

    pub fn reset_embedded(&mut self) {
        self.r#embedded = None;
    }

    pub fn set_links(&mut self, r#links: Value) {
        self.r#links = Some(r#links);
    }

    pub fn with_links(mut self, r#links: Value) -> Self {
        self.r#links = Some(r#links);
        self
    }

    pub fn r#links(&self) -> Option<&Value> {
        self.r#links.as_ref().map(|x| x.borrow())
    }

    pub fn reset_links(&mut self) {
        self.r#links = None;
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

    pub fn set_last_updated(&mut self, r#last_updated: String) {
        self.r#last_updated = Some(r#last_updated);
    }

    pub fn with_last_updated(mut self, r#last_updated: String) -> Self {
        self.r#last_updated = Some(r#last_updated);
        self
    }

    pub fn r#last_updated(&self) -> Option<&str> {
        self.r#last_updated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_updated(&mut self) {
        self.r#last_updated = None;
    }

    pub fn set_priority(&mut self, r#priority: i32) {
        self.r#priority = Some(r#priority);
    }

    pub fn with_priority(mut self, r#priority: i32) -> Self {
        self.r#priority = Some(r#priority);
        self
    }

    pub fn r#priority(&self) -> Option<&i32> {
        self.r#priority.as_ref().map(|x| x.borrow())
    }

    pub fn reset_priority(&mut self) {
        self.r#priority = None;
    }

    pub fn set_profile(&mut self, r#profile: Value) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: Value) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&Value> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }
}
