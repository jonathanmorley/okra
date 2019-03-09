#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SecurePasswordStoreApplicationSettings {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    r#app: Option<SecurePasswordStoreApplicationSettingsApplication>,
}

impl r#SecurePasswordStoreApplicationSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#app: None,
        }
    }

    pub fn set_app(&mut self, r#app: SecurePasswordStoreApplicationSettingsApplication) {
        self.r#app = Some(r#app);
    }

    pub fn with_app(mut self, r#app: SecurePasswordStoreApplicationSettingsApplication) -> Self {
        self.r#app = Some(r#app);
        self
    }

    pub fn r#app(&self) -> Option<&SecurePasswordStoreApplicationSettingsApplication> {
        self.r#app.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app(&mut self) {
        self.r#app = None;
    }
}
