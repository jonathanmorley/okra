#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#U2fFactor {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<U2fFactorProfile>,
}

impl r#U2fFactor {
    pub fn new(
    ) -> Self {
        Self {
          r#profile: None,
        }
    }

    pub fn set_profile(&mut self, r#profile: U2fFactorProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: U2fFactorProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&U2fFactorProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }
}
