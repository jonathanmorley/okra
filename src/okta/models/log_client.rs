#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogClient {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    r#device: Option<String>,
    #[serde(rename = "geographicalContext", skip_serializing_if = "Option::is_none")]
    r#geographical_context: Option<LogGeographicalContext>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    r#ip_address: Option<String>,
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    r#user_agent: Option<LogUserAgent>,
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    r#zone: Option<String>,
}

impl r#LogClient {
    pub fn new(
    ) -> Self {
        Self {
          r#device: None,
          r#geographical_context: None,
          r#id: None,
          r#ip_address: None,
          r#user_agent: None,
          r#zone: None,
        }
    }

    pub fn set_device(&mut self, r#device: String) {
        self.r#device = Some(r#device);
    }

    pub fn with_device(mut self, r#device: String) -> Self {
        self.r#device = Some(r#device);
        self
    }

    pub fn r#device(&self) -> Option<&str> {
        self.r#device.as_ref().map(|x| x.borrow())
    }

    pub fn reset_device(&mut self) {
        self.r#device = None;
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

    pub fn set_id(&mut self, r#id: String) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: String) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn r#id(&self) -> Option<&str> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
    }

    pub fn set_ip_address(&mut self, r#ip_address: String) {
        self.r#ip_address = Some(r#ip_address);
    }

    pub fn with_ip_address(mut self, r#ip_address: String) -> Self {
        self.r#ip_address = Some(r#ip_address);
        self
    }

    pub fn r#ip_address(&self) -> Option<&str> {
        self.r#ip_address.as_ref().map(|x| x.borrow())
    }

    pub fn reset_ip_address(&mut self) {
        self.r#ip_address = None;
    }

    pub fn set_user_agent(&mut self, r#user_agent: LogUserAgent) {
        self.r#user_agent = Some(r#user_agent);
    }

    pub fn with_user_agent(mut self, r#user_agent: LogUserAgent) -> Self {
        self.r#user_agent = Some(r#user_agent);
        self
    }

    pub fn r#user_agent(&self) -> Option<&LogUserAgent> {
        self.r#user_agent.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_agent(&mut self) {
        self.r#user_agent = None;
    }

    pub fn set_zone(&mut self, r#zone: String) {
        self.r#zone = Some(r#zone);
    }

    pub fn with_zone(mut self, r#zone: String) -> Self {
        self.r#zone = Some(r#zone);
        self
    }

    pub fn r#zone(&self) -> Option<&str> {
        self.r#zone.as_ref().map(|x| x.borrow())
    }

    pub fn reset_zone(&mut self) {
        self.r#zone = None;
    }
}
