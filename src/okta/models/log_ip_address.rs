#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogIpAddress {
    #[serde(rename = "geographicalContext", skip_serializing_if = "Option::is_none")]
    r#geographical_context: Option<LogGeographicalContext>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    r#ip: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    r#source: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    r#version: Option<String>,
}

impl r#LogIpAddress {
    pub fn new(
    ) -> Self {
        Self {
          r#geographical_context: None,
          r#ip: None,
          r#source: None,
          r#version: None,
        }
    }

    pub fn set_geographical_context(&mut self, r#geographical_context: LogGeographicalContext) {
        self.r#geographical_context = Some(r#geographical_context);
    }

    pub fn with_geographical_context(mut self, r#geographical_context: LogGeographicalContext) -> Self {
        self.r#geographical_context = Some(r#geographical_context);
        self
    }

    pub fn r#geographical_context(&self) -> Option<&LogGeographicalContext> {
        self.r#geographical_context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_geographical_context(&mut self) {
        self.r#geographical_context = None;
    }

    pub fn set_ip(&mut self, r#ip: String) {
        self.r#ip = Some(r#ip);
    }

    pub fn with_ip(mut self, r#ip: String) -> Self {
        self.r#ip = Some(r#ip);
        self
    }

    pub fn r#ip(&self) -> Option<&str> {
        self.r#ip.as_ref().map(|x| x.borrow())
    }

    pub fn reset_ip(&mut self) {
        self.r#ip = None;
    }

    pub fn set_source(&mut self, r#source: String) {
        self.r#source = Some(r#source);
    }

    pub fn with_source(mut self, r#source: String) -> Self {
        self.r#source = Some(r#source);
        self
    }

    pub fn r#source(&self) -> Option<&str> {
        self.r#source.as_ref().map(|x| x.borrow())
    }

    pub fn reset_source(&mut self) {
        self.r#source = None;
    }

    pub fn set_version(&mut self, r#version: String) {
        self.r#version = Some(r#version);
    }

    pub fn with_version(mut self, r#version: String) -> Self {
        self.r#version = Some(r#version);
        self
    }

    pub fn r#version(&self) -> Option<&str> {
        self.r#version.as_ref().map(|x| x.borrow())
    }

    pub fn reset_version(&mut self) {
        self.r#version = None;
    }
}
