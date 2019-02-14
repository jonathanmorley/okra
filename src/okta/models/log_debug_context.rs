#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogDebugContext {
    #[serde(rename = "debugData", skip_serializing_if = "Option::is_none")]
    r#debug_data: Option<Value>,
}

impl r#LogDebugContext {
    pub fn new(
    ) -> Self {
        Self {
          r#debug_data: None,
        }
    }

    pub fn set_debug_data(&mut self, r#debug_data: Value) {
        self.r#debug_data = Some(r#debug_data);
    }

    pub fn with_debug_data(mut self, r#debug_data: Value) -> Self {
        self.r#debug_data = Some(r#debug_data);
        self
    }

    pub fn r#debug_data(&self) -> Option<&Value> {
        self.r#debug_data.as_ref().map(|x| x.borrow())
    }

    pub fn reset_debug_data(&mut self) {
        self.r#debug_data = None;
    }
}
