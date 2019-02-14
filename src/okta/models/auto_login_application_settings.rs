#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AutoLoginApplicationSettings {
    #[serde(rename = "signOn", skip_serializing_if = "Option::is_none")]
    r#sign_on: Option<AutoLoginApplicationSettingsSignOn>,
}

impl r#AutoLoginApplicationSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#sign_on: None,
        }
    }

    pub fn set_sign_on(&mut self, r#sign_on: AutoLoginApplicationSettingsSignOn) {
        self.r#sign_on = Some(r#sign_on);
    }

    pub fn with_sign_on(mut self, r#sign_on: AutoLoginApplicationSettingsSignOn) -> Self {
        self.r#sign_on = Some(r#sign_on);
        self
    }

    pub fn r#sign_on(&self) -> Option<&AutoLoginApplicationSettingsSignOn> {
        self.r#sign_on.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sign_on(&mut self) {
        self.r#sign_on = None;
    }
}
