#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#TotpFactor {
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<TotpFactorProfile>,
}

impl r#TotpFactor {
    pub fn new(
    ) -> Self {
        Self {
          r#profile: None,
        }
    }

    pub fn set_profile(&mut self, r#profile: TotpFactorProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: TotpFactorProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&TotpFactorProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }
}
