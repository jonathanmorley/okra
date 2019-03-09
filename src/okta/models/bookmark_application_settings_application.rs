#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#BookmarkApplicationSettingsApplication {
    #[serde(rename = "requestIntegration", skip_serializing_if = "Option::is_none")]
    r#request_integration: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    r#url: Option<String>,
}

impl r#BookmarkApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#request_integration: None,
          r#url: None,
        }
    }

    pub fn set_request_integration(&mut self, r#request_integration: bool) {
        self.r#request_integration = Some(r#request_integration);
    }

    pub fn with_request_integration(mut self, r#request_integration: bool) -> Self {
        self.r#request_integration = Some(r#request_integration);
        self
    }

    pub fn r#request_integration(&self) -> Option<&bool> {
        self.r#request_integration.as_ref().map(|x| x.borrow())
    }

    pub fn reset_request_integration(&mut self) {
        self.r#request_integration = None;
    }

    pub fn set_url(&mut self, r#url: String) {
        self.r#url = Some(r#url);
    }

    pub fn with_url(mut self, r#url: String) -> Self {
        self.r#url = Some(r#url);
        self
    }

    pub fn r#url(&self) -> Option<&str> {
        self.r#url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_url(&mut self) {
        self.r#url = None;
    }
}
