#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationSettingsNotificationsVpn {
    #[serde(rename = "helpUrl", skip_serializing_if = "Option::is_none")]
    r#help_url: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    r#message: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    r#network: Option<ApplicationSettingsNotificationsVpnNetwork>,
}

impl r#ApplicationSettingsNotificationsVpn {
    pub fn new(
    ) -> Self {
        Self {
          r#help_url: None,
          r#message: None,
          r#network: None,
        }
    }

    pub fn set_help_url(&mut self, r#help_url: String) {
        self.r#help_url = Some(r#help_url);
    }

    pub fn with_help_url(mut self, r#help_url: String) -> Self {
        self.r#help_url = Some(r#help_url);
        self
    }

    pub fn r#help_url(&self) -> Option<&str> {
        self.r#help_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_help_url(&mut self) {
        self.r#help_url = None;
    }

    pub fn set_message(&mut self, r#message: String) {
        self.r#message = Some(r#message);
    }

    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = Some(r#message);
        self
    }

    pub fn r#message(&self) -> Option<&str> {
        self.r#message.as_ref().map(|x| x.borrow())
    }

    pub fn reset_message(&mut self) {
        self.r#message = None;
    }

    pub fn set_network(&mut self, r#network: ApplicationSettingsNotificationsVpnNetwork) {
        self.r#network = Some(r#network);
    }

    pub fn with_network(mut self, r#network: ApplicationSettingsNotificationsVpnNetwork) -> Self {
        self.r#network = Some(r#network);
        self
    }

    pub fn r#network(&self) -> Option<&ApplicationSettingsNotificationsVpnNetwork> {
        self.r#network.as_ref().map(|x| x.borrow())
    }

    pub fn reset_network(&mut self) {
        self.r#network = None;
    }
}
