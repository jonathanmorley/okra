#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRuleExpression {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    r#value: Option<String>,
}

impl r#GroupRuleExpression {
    pub fn new(
    ) -> Self {
        Self {
          r#type: None,
          r#value: None,
        }
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

    pub fn set_value(&mut self, r#value: String) {
        self.r#value = Some(r#value);
    }

    pub fn with_value(mut self, r#value: String) -> Self {
        self.r#value = Some(r#value);
        self
    }

    pub fn r#value(&self) -> Option<&str> {
        self.r#value.as_ref().map(|x| x.borrow())
    }

    pub fn reset_value(&mut self) {
        self.r#value = None;
    }
}
