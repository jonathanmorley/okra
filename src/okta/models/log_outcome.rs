#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogOutcome {
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    r#reason: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    r#result: Option<String>,
}

impl r#LogOutcome {
    pub fn new(
    ) -> Self {
        Self {
          r#reason: None,
          r#result: None,
        }
    }

    pub fn set_reason(&mut self, r#reason: String) {
        self.r#reason = Some(r#reason);
    }

    pub fn with_reason(mut self, r#reason: String) -> Self {
        self.r#reason = Some(r#reason);
        self
    }

    pub fn r#reason(&self) -> Option<&str> {
        self.r#reason.as_ref().map(|x| x.borrow())
    }

    pub fn reset_reason(&mut self) {
        self.r#reason = None;
    }

    pub fn set_result(&mut self, r#result: String) {
        self.r#result = Some(r#result);
    }

    pub fn with_result(mut self, r#result: String) -> Self {
        self.r#result = Some(r#result);
        self
    }

    pub fn r#result(&self) -> Option<&str> {
        self.r#result.as_ref().map(|x| x.borrow())
    }

    pub fn reset_result(&mut self) {
        self.r#result = None;
    }
}
