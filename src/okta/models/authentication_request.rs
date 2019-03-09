#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AuthenticationRequest {
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    r#audience: Option<String>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    r#context: Option<Context>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    r#options: Option<Options>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    r#password: Option<String>,
    #[serde(rename = "relayState", skip_serializing_if = "Option::is_none")]
    r#relay_state: Option<String>,
    #[serde(rename = "stateToken", skip_serializing_if = "Option::is_none")]
    r#state_token: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    r#token: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    r#username: Option<String>,
}

impl r#AuthenticationRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#audience: None,
          r#context: None,
          r#options: None,
          r#password: None,
          r#relay_state: None,
          r#state_token: None,
          r#token: None,
          r#username: None,
        }
    }

    pub fn set_audience(&mut self, r#audience: String) {
        self.r#audience = Some(r#audience);
    }

    pub fn with_audience(mut self, r#audience: String) -> Self {
        self.r#audience = Some(r#audience);
        self
    }

    pub fn r#audience(&self) -> Option<&str> {
        self.r#audience.as_ref().map(|x| x.borrow())
    }

    pub fn reset_audience(&mut self) {
        self.r#audience = None;
    }

    pub fn set_context(&mut self, r#context: Context) {
        self.r#context = Some(r#context);
    }

    pub fn with_context(mut self, r#context: Context) -> Self {
        self.r#context = Some(r#context);
        self
    }

    pub fn r#context(&self) -> Option<&Context> {
        self.r#context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_context(&mut self) {
        self.r#context = None;
    }

    pub fn set_options(&mut self, r#options: Options) {
        self.r#options = Some(r#options);
    }

    pub fn with_options(mut self, r#options: Options) -> Self {
        self.r#options = Some(r#options);
        self
    }

    pub fn r#options(&self) -> Option<&Options> {
        self.r#options.as_ref().map(|x| x.borrow())
    }

    pub fn reset_options(&mut self) {
        self.r#options = None;
    }

    pub fn set_password(&mut self, r#password: String) {
        self.r#password = Some(r#password);
    }

    pub fn with_password(mut self, r#password: String) -> Self {
        self.r#password = Some(r#password);
        self
    }

    pub fn r#password(&self) -> Option<&str> {
        self.r#password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password(&mut self) {
        self.r#password = None;
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

    pub fn set_token(&mut self, r#token: String) {
        self.r#token = Some(r#token);
    }

    pub fn with_token(mut self, r#token: String) -> Self {
        self.r#token = Some(r#token);
        self
    }

    pub fn r#token(&self) -> Option<&str> {
        self.r#token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_token(&mut self) {
        self.r#token = None;
    }

    pub fn set_username(&mut self, r#username: String) {
        self.r#username = Some(r#username);
    }

    pub fn with_username(mut self, r#username: String) -> Self {
        self.r#username = Some(r#username);
        self
    }

    pub fn r#username(&self) -> Option<&str> {
        self.r#username.as_ref().map(|x| x.borrow())
    }

    pub fn reset_username(&mut self) {
        self.r#username = None;
    }
}
