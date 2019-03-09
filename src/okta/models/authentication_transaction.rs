#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AuthenticationTransaction {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    r#expires_at: Option<String>,
    #[serde(rename = "factorResult", skip_serializing_if = "Option::is_none")]
    r#factor_result: Option<FactorResultType>,
    #[serde(rename = "relayState", skip_serializing_if = "Option::is_none")]
    r#relay_state: Option<String>,
    #[serde(rename = "sessionToken", skip_serializing_if = "Option::is_none")]
    r#session_token: Option<String>,
    #[serde(rename = "stateToken", skip_serializing_if = "Option::is_none")]
    r#state_token: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<TransactionState>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#AuthenticationTransaction {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#expires_at: None,
          r#factor_result: None,
          r#relay_state: None,
          r#session_token: None,
          r#state_token: None,
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

    pub fn set_relay_state(&mut self, r#relay_state: String) {
        self.r#relay_state = Some(r#relay_state);
    }

    pub fn with_relay_state(mut self, r#relay_state: String) -> Self {
        self.r#relay_state = Some(r#relay_state);
        self
    }

    pub fn r#relay_state(&self) -> Option<&str> {
        self.r#relay_state.as_ref().map(|x| x.borrow())
    }

    pub fn reset_relay_state(&mut self) {
        self.r#relay_state = None;
    }

    pub fn set_session_token(&mut self, r#session_token: String) {
        self.r#session_token = Some(r#session_token);
    }

    pub fn with_session_token(mut self, r#session_token: String) -> Self {
        self.r#session_token = Some(r#session_token);
        self
    }

    pub fn r#session_token(&self) -> Option<&str> {
        self.r#session_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_session_token(&mut self) {
        self.r#session_token = None;
    }

    pub fn set_state_token(&mut self, r#state_token: String) {
        self.r#state_token = Some(r#state_token);
    }

    pub fn with_state_token(mut self, r#state_token: String) -> Self {
        self.r#state_token = Some(r#state_token);
        self
    }

    pub fn r#state_token(&self) -> Option<&str> {
        self.r#state_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_state_token(&mut self) {
        self.r#state_token = None;
    }

    pub fn set_status(&mut self, r#status: TransactionState) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: TransactionState) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&TransactionState> {
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
