#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SamlApplication {
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    r#settings: Option<SamlApplicationSettings>,
}

impl r#SamlApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#settings: None,
        }
    }

    pub fn set_settings(&mut self, r#settings: SamlApplicationSettings) {
        self.r#settings = Some(r#settings);
    }

    pub fn with_settings(mut self, r#settings: SamlApplicationSettings) -> Self {
        self.r#settings = Some(r#settings);
        self
    }

    pub fn r#settings(&self) -> Option<&SamlApplicationSettings> {
        self.r#settings.as_ref().map(|x| x.borrow())
    }

    pub fn reset_settings(&mut self) {
        self.r#settings = None;
    }
}
