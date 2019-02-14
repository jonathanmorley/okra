#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Role {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    r#description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    r#label: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<RoleStatus>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#Role {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#created: None,
          r#description: None,
          r#id: None,
          r#label: None,
          r#last_updated: None,
          r#status: None,
          r#type: None,
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

    pub fn set_created(&mut self, r#created: String) {
        self.r#created = Some(r#created);
    }

    pub fn with_created(mut self, r#created: String) -> Self {
        self.r#created = Some(r#created);
        self
    }

    pub fn r#created(&self) -> Option<&str> {
        self.r#created.as_ref().map(|x| x.borrow())
    }

    pub fn reset_created(&mut self) {
        self.r#created = None;
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

    pub fn set_label(&mut self, r#label: String) {
        self.r#label = Some(r#label);
    }

    pub fn with_label(mut self, r#label: String) -> Self {
        self.r#label = Some(r#label);
        self
    }

    pub fn r#label(&self) -> Option<&str> {
        self.r#label.as_ref().map(|x| x.borrow())
    }

    pub fn reset_label(&mut self) {
        self.r#label = None;
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

    pub fn set_status(&mut self, r#status: RoleStatus) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: RoleStatus) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&RoleStatus> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
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
