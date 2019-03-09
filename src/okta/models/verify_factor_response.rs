#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#VerifyFactorResponse {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    r#expires_at: Option<String>,
    #[serde(rename = "factorResult", skip_serializing_if = "Option::is_none")]
    r#factor_result: Option<FactorResultType>,
    #[serde(rename = "factorResultMessage", skip_serializing_if = "Option::is_none")]
    r#factor_result_message: Option<String>,
}

impl r#VerifyFactorResponse {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#expires_at: None,
          r#factor_result: None,
          r#factor_result_message: None,
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

    pub fn set_expires_at(&mut self, r#expires_at: String) {
        self.r#expires_at = Some(r#expires_at);
    }

    pub fn with_expires_at(mut self, r#expires_at: String) -> Self {
        self.r#expires_at = Some(r#expires_at);
        self
    }

    pub fn r#expires_at(&self) -> Option<&str> {
        self.r#expires_at.as_ref().map(|x| x.borrow())
    }

    pub fn reset_expires_at(&mut self) {
        self.r#expires_at = None;
    }

    pub fn set_factor_result(&mut self, r#factor_result: FactorResultType) {
        self.r#factor_result = Some(r#factor_result);
    }

    pub fn with_factor_result(mut self, r#factor_result: FactorResultType) -> Self {
        self.r#factor_result = Some(r#factor_result);
        self
    }

    pub fn r#factor_result(&self) -> Option<&FactorResultType> {
        self.r#factor_result.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factor_result(&mut self) {
        self.r#factor_result = None;
    }

    pub fn set_factor_result_message(&mut self, r#factor_result_message: String) {
        self.r#factor_result_message = Some(r#factor_result_message);
    }

    pub fn with_factor_result_message(mut self, r#factor_result_message: String) -> Self {
        self.r#factor_result_message = Some(r#factor_result_message);
        self
    }

    pub fn r#factor_result_message(&self) -> Option<&str> {
        self.r#factor_result_message.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factor_result_message(&mut self) {
        self.r#factor_result_message = None;
    }
}
