#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PushFactorProfile {
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    r#credential_id: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    r#device_type: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    r#platform: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    r#version: Option<String>,
}

impl r#PushFactorProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#credential_id: None,
          r#device_type: None,
          r#name: None,
          r#platform: None,
          r#version: None,
        }
    }

    pub fn set_credential_id(&mut self, r#credential_id: String) {
        self.r#credential_id = Some(r#credential_id);
    }

    pub fn with_credential_id(mut self, r#credential_id: String) -> Self {
        self.r#credential_id = Some(r#credential_id);
        self
    }

    pub fn r#credential_id(&self) -> Option<&str> {
        self.r#credential_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credential_id(&mut self) {
        self.r#credential_id = None;
    }

    pub fn set_device_type(&mut self, r#device_type: String) {
        self.r#device_type = Some(r#device_type);
    }

    pub fn with_device_type(mut self, r#device_type: String) -> Self {
        self.r#device_type = Some(r#device_type);
        self
    }

    pub fn r#device_type(&self) -> Option<&str> {
        self.r#device_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_device_type(&mut self) {
        self.r#device_type = None;
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = Some(r#name);
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = Some(r#name);
        self
    }

    pub fn r#name(&self) -> Option<&str> {
        self.r#name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name(&mut self) {
        self.r#name = None;
    }

    pub fn set_platform(&mut self, r#platform: String) {
        self.r#platform = Some(r#platform);
    }

    pub fn with_platform(mut self, r#platform: String) -> Self {
        self.r#platform = Some(r#platform);
        self
    }

    pub fn r#platform(&self) -> Option<&str> {
        self.r#platform.as_ref().map(|x| x.borrow())
    }

    pub fn reset_platform(&mut self) {
        self.r#platform = None;
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
