#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyAuthenticationProviderCondition {
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    r#include: Option<Vec<String>>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    r#provider: Option<String>,
}

impl r#PasswordPolicyAuthenticationProviderCondition {
    pub fn new(
    ) -> Self {
        Self {
          r#include: None,
          r#provider: None,
        }
    }

    pub fn set_include(&mut self, r#include: Vec<String>) {
        self.r#include = Some(r#include);
    }

    pub fn with_include(mut self, r#include: Vec<String>) -> Self {
        self.r#include = Some(r#include);
        self
    }

    pub fn r#include(&self) -> Option<&Vec<String>> {
        self.r#include.as_ref().map(|x| x.borrow())
    }

    pub fn reset_include(&mut self) {
        self.r#include = None;
    }

    pub fn set_provider(&mut self, r#provider: String) {
        self.r#provider = Some(r#provider);
    }

    pub fn with_provider(mut self, r#provider: String) -> Self {
        self.r#provider = Some(r#provider);
        self
    }

    pub fn r#provider(&self) -> Option<&str> {
        self.r#provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_provider(&mut self) {
        self.r#provider = None;
    }
}
