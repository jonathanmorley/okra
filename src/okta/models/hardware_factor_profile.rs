#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#HardwareFactorProfile {
    #[serde(rename = "credentialId", skip_serializing_if = "Option::is_none")]
    r#credential_id: Option<String>,
}

impl r#HardwareFactorProfile {
    pub fn new(
    ) -> Self {
        Self {
          r#credential_id: None,
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
}
