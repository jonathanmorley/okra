#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationSettingsNotificationsVpnNetwork {
    #[serde(rename = "connection", skip_serializing_if = "Option::is_none")]
    r#connection: Option<String>,
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    r#exclude: Option<Vec<String>>,
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    r#include: Option<Vec<String>>,
}

impl r#ApplicationSettingsNotificationsVpnNetwork {
    pub fn new(
    ) -> Self {
        Self {
          r#connection: None,
          r#exclude: None,
          r#include: None,
        }
    }

    pub fn set_connection(&mut self, r#connection: String) {
        self.r#connection = Some(r#connection);
    }

    pub fn with_connection(mut self, r#connection: String) -> Self {
        self.r#connection = Some(r#connection);
        self
    }

    pub fn r#connection(&self) -> Option<&str> {
        self.r#connection.as_ref().map(|x| x.borrow())
    }

    pub fn reset_connection(&mut self) {
        self.r#connection = None;
    }

    pub fn set_exclude(&mut self, r#exclude: Vec<String>) {
        self.r#exclude = Some(r#exclude);
    }

    pub fn with_exclude(mut self, r#exclude: Vec<String>) -> Self {
        self.r#exclude = Some(r#exclude);
        self
    }

    pub fn r#exclude(&self) -> Option<&Vec<String>> {
        self.r#exclude.as_ref().map(|x| x.borrow())
    }

    pub fn reset_exclude(&mut self) {
        self.r#exclude = None;
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
}
