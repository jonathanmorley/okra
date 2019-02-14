#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#BrowserPluginApplication {
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    r#credentials: Option<SchemeApplicationCredentials>,
}

impl r#BrowserPluginApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#credentials: None,
        }
    }

    pub fn set_credentials(&mut self, r#credentials: SchemeApplicationCredentials) {
        self.r#credentials = Some(r#credentials);
    }

    pub fn with_credentials(mut self, r#credentials: SchemeApplicationCredentials) -> Self {
        self.r#credentials = Some(r#credentials);
        self
    }

    pub fn r#credentials(&self) -> Option<&SchemeApplicationCredentials> {
        self.r#credentials.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credentials(&mut self) {
        self.r#credentials = None;
    }
}
