#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#VerifyFactorRequest {
    #[serde(rename = "activationToken", skip_serializing_if = "Option::is_none")]
    r#activation_token: Option<String>,
    #[serde(rename = "answer", skip_serializing_if = "Option::is_none")]
    r#answer: Option<String>,
    #[serde(rename = "nextPassCode", skip_serializing_if = "Option::is_none")]
    r#next_pass_code: Option<String>,
    #[serde(rename = "passCode", skip_serializing_if = "Option::is_none")]
    r#pass_code: Option<String>,
    #[serde(rename = "tokenLifetimeSeconds", skip_serializing_if = "Option::is_none")]
    r#token_lifetime_seconds: Option<i32>,
}

impl r#VerifyFactorRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#activation_token: None,
          r#answer: None,
          r#next_pass_code: None,
          r#pass_code: None,
          r#token_lifetime_seconds: None,
        }
    }

    pub fn set_activation_token(&mut self, r#activation_token: String) {
        self.r#activation_token = Some(r#activation_token);
    }

    pub fn with_activation_token(mut self, r#activation_token: String) -> Self {
        self.r#activation_token = Some(r#activation_token);
        self
    }

    pub fn r#activation_token(&self) -> Option<&str> {
        self.r#activation_token.as_ref().map(|x| x.borrow())
    }

    pub fn reset_activation_token(&mut self) {
        self.r#activation_token = None;
    }

    pub fn set_answer(&mut self, r#answer: String) {
        self.r#answer = Some(r#answer);
    }

    pub fn with_answer(mut self, r#answer: String) -> Self {
        self.r#answer = Some(r#answer);
        self
    }

    pub fn r#answer(&self) -> Option<&str> {
        self.r#answer.as_ref().map(|x| x.borrow())
    }

    pub fn reset_answer(&mut self) {
        self.r#answer = None;
    }

    pub fn set_next_pass_code(&mut self, r#next_pass_code: String) {
        self.r#next_pass_code = Some(r#next_pass_code);
    }

    pub fn with_next_pass_code(mut self, r#next_pass_code: String) -> Self {
        self.r#next_pass_code = Some(r#next_pass_code);
        self
    }

    pub fn r#next_pass_code(&self) -> Option<&str> {
        self.r#next_pass_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_next_pass_code(&mut self) {
        self.r#next_pass_code = None;
    }

    pub fn set_pass_code(&mut self, r#pass_code: String) {
        self.r#pass_code = Some(r#pass_code);
    }

    pub fn with_pass_code(mut self, r#pass_code: String) -> Self {
        self.r#pass_code = Some(r#pass_code);
        self
    }

    pub fn r#pass_code(&self) -> Option<&str> {
        self.r#pass_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_pass_code(&mut self) {
        self.r#pass_code = None;
    }

    pub fn set_token_lifetime_seconds(&mut self, r#token_lifetime_seconds: i32) {
        self.r#token_lifetime_seconds = Some(r#token_lifetime_seconds);
    }

    pub fn with_token_lifetime_seconds(mut self, r#token_lifetime_seconds: i32) -> Self {
        self.r#token_lifetime_seconds = Some(r#token_lifetime_seconds);
        self
    }

    pub fn r#token_lifetime_seconds(&self) -> Option<&i32> {
        self.r#token_lifetime_seconds.as_ref().map(|x| x.borrow())
    }

    pub fn reset_token_lifetime_seconds(&mut self) {
        self.r#token_lifetime_seconds = None;
    }
}
