#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyRuleSignonSessionActions {
    #[serde(rename = "maxSessionIdleMinutes", skip_serializing_if = "Option::is_none")]
    r#max_session_idle_minutes: Option<i32>,
    #[serde(rename = "maxSessionLifetimeMinutes", skip_serializing_if = "Option::is_none")]
    r#max_session_lifetime_minutes: Option<i32>,
    #[serde(rename = "usePersistentCookie", skip_serializing_if = "Option::is_none")]
    r#use_persistent_cookie: Option<bool>,
}

impl r#OktaSignOnPolicyRuleSignonSessionActions {
    pub fn new(
    ) -> Self {
        Self {
          r#max_session_idle_minutes: None,
          r#max_session_lifetime_minutes: None,
          r#use_persistent_cookie: None,
        }
    }

    pub fn set_max_session_idle_minutes(&mut self, r#max_session_idle_minutes: i32) {
        self.r#max_session_idle_minutes = Some(r#max_session_idle_minutes);
    }

    pub fn with_max_session_idle_minutes(mut self, r#max_session_idle_minutes: i32) -> Self {
        self.r#max_session_idle_minutes = Some(r#max_session_idle_minutes);
        self
    }

    pub fn r#max_session_idle_minutes(&self) -> Option<&i32> {
        self.r#max_session_idle_minutes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_max_session_idle_minutes(&mut self) {
        self.r#max_session_idle_minutes = None;
    }

    pub fn set_max_session_lifetime_minutes(&mut self, r#max_session_lifetime_minutes: i32) {
        self.r#max_session_lifetime_minutes = Some(r#max_session_lifetime_minutes);
    }

    pub fn with_max_session_lifetime_minutes(mut self, r#max_session_lifetime_minutes: i32) -> Self {
        self.r#max_session_lifetime_minutes = Some(r#max_session_lifetime_minutes);
        self
    }

    pub fn r#max_session_lifetime_minutes(&self) -> Option<&i32> {
        self.r#max_session_lifetime_minutes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_max_session_lifetime_minutes(&mut self) {
        self.r#max_session_lifetime_minutes = None;
    }

    pub fn set_use_persistent_cookie(&mut self, r#use_persistent_cookie: bool) {
        self.r#use_persistent_cookie = Some(r#use_persistent_cookie);
    }

    pub fn with_use_persistent_cookie(mut self, r#use_persistent_cookie: bool) -> Self {
        self.r#use_persistent_cookie = Some(r#use_persistent_cookie);
        self
    }

    pub fn r#use_persistent_cookie(&self) -> Option<&bool> {
        self.r#use_persistent_cookie.as_ref().map(|x| x.borrow())
    }

    pub fn reset_use_persistent_cookie(&mut self) {
        self.r#use_persistent_cookie = None;
    }
}
