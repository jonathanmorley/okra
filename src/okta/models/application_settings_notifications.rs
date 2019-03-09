#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationSettingsNotifications {
    #[serde(rename = "vpn", skip_serializing_if = "Option::is_none")]
    r#vpn: Option<ApplicationSettingsNotificationsVpn>,
}

impl r#ApplicationSettingsNotifications {
    pub fn new(
    ) -> Self {
        Self {
          r#vpn: None,
        }
    }

    pub fn set_vpn(&mut self, r#vpn: ApplicationSettingsNotificationsVpn) {
        self.r#vpn = Some(r#vpn);
    }

    pub fn with_vpn(mut self, r#vpn: ApplicationSettingsNotificationsVpn) -> Self {
        self.r#vpn = Some(r#vpn);
        self
    }

    pub fn r#vpn(&self) -> Option<&ApplicationSettingsNotificationsVpn> {
        self.r#vpn.as_ref().map(|x| x.borrow())
    }

    pub fn reset_vpn(&mut self) {
        self.r#vpn = None;
    }
}
