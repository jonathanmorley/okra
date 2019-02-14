#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Options {
    #[serde(rename = "multiOptionalFactorEnroll", skip_serializing_if = "Option::is_none")]
    r#multi_optional_factor_enroll: Option<bool>,
    #[serde(rename = "warnBeforePasswordExpired", skip_serializing_if = "Option::is_none")]
    r#warn_before_password_expired: Option<bool>,
}

impl r#Options {
    pub fn new(
    ) -> Self {
        Self {
          r#multi_optional_factor_enroll: None,
          r#warn_before_password_expired: None,
        }
    }

    pub fn set_multi_optional_factor_enroll(&mut self, r#multi_optional_factor_enroll: bool) {
        self.r#multi_optional_factor_enroll = Some(r#multi_optional_factor_enroll);
    }

    pub fn with_multi_optional_factor_enroll(mut self, r#multi_optional_factor_enroll: bool) -> Self {
        self.r#multi_optional_factor_enroll = Some(r#multi_optional_factor_enroll);
        self
    }

    pub fn r#multi_optional_factor_enroll(&self) -> Option<&bool> {
        self.r#multi_optional_factor_enroll.as_ref().map(|x| x.borrow())
    }

    pub fn reset_multi_optional_factor_enroll(&mut self) {
        self.r#multi_optional_factor_enroll = None;
    }

    pub fn set_warn_before_password_expired(&mut self, r#warn_before_password_expired: bool) {
        self.r#warn_before_password_expired = Some(r#warn_before_password_expired);
    }

    pub fn with_warn_before_password_expired(mut self, r#warn_before_password_expired: bool) -> Self {
        self.r#warn_before_password_expired = Some(r#warn_before_password_expired);
        self
    }

    pub fn r#warn_before_password_expired(&self) -> Option<&bool> {
        self.r#warn_before_password_expired.as_ref().map(|x| x.borrow())
    }

    pub fn reset_warn_before_password_expired(&mut self) {
        self.r#warn_before_password_expired = None;
    }
}
