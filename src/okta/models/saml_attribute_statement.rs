#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SamlAttributeStatement {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    r#namespace: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    r#values: Option<Vec<String>>,
}

impl r#SamlAttributeStatement {
    pub fn new(
    ) -> Self {
        Self {
          r#name: None,
          r#namespace: None,
          r#type: None,
          r#values: None,
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

    pub fn set_namespace(&mut self, r#namespace: String) {
        self.r#namespace = Some(r#namespace);
    }

    pub fn with_namespace(mut self, r#namespace: String) -> Self {
        self.r#namespace = Some(r#namespace);
        self
    }

    pub fn r#namespace(&self) -> Option<&str> {
        self.r#namespace.as_ref().map(|x| x.borrow())
    }

    pub fn reset_namespace(&mut self) {
        self.r#namespace = None;
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

    pub fn set_values(&mut self, r#values: Vec<String>) {
        self.r#values = Some(r#values);
    }

    pub fn with_values(mut self, r#values: Vec<String>) -> Self {
        self.r#values = Some(r#values);
        self
    }

    pub fn r#values(&self) -> Option<&Vec<String>> {
        self.r#values.as_ref().map(|x| x.borrow())
    }

    pub fn reset_values(&mut self) {
        self.r#values = None;
    }
}
