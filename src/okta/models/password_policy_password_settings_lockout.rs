#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyPasswordSettingsLockout {
    #[serde(rename = "autoUnlockMinutes", skip_serializing_if = "Option::is_none")]
    r#auto_unlock_minutes: Option<i32>,
    #[serde(rename = "maxAttempts", skip_serializing_if = "Option::is_none")]
    r#max_attempts: Option<i32>,
    #[serde(rename = "showLockoutFailures", skip_serializing_if = "Option::is_none")]
    r#show_lockout_failures: Option<bool>,
    #[serde(rename = "userLockoutNotificationChannels", skip_serializing_if = "Option::is_none")]
    r#user_lockout_notification_channels: Option<Vec<String>>,
}

impl r#PasswordPolicyPasswordSettingsLockout {
    pub fn new(
    ) -> Self {
        Self {
          r#auto_unlock_minutes: None,
          r#max_attempts: None,
          r#show_lockout_failures: None,
          r#user_lockout_notification_channels: None,
        }
    }

    pub fn set_auto_unlock_minutes(&mut self, r#auto_unlock_minutes: i32) {
        self.r#auto_unlock_minutes = Some(r#auto_unlock_minutes);
    }

    pub fn with_auto_unlock_minutes(mut self, r#auto_unlock_minutes: i32) -> Self {
        self.r#auto_unlock_minutes = Some(r#auto_unlock_minutes);
        self
    }

    pub fn r#auto_unlock_minutes(&self) -> Option<&i32> {
        self.r#auto_unlock_minutes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auto_unlock_minutes(&mut self) {
        self.r#auto_unlock_minutes = None;
    }

    pub fn set_max_attempts(&mut self, r#max_attempts: i32) {
        self.r#max_attempts = Some(r#max_attempts);
    }

    pub fn with_max_attempts(mut self, r#max_attempts: i32) -> Self {
        self.r#max_attempts = Some(r#max_attempts);
        self
    }

    pub fn r#max_attempts(&self) -> Option<&i32> {
        self.r#max_attempts.as_ref().map(|x| x.borrow())
    }

    pub fn reset_max_attempts(&mut self) {
        self.r#max_attempts = None;
    }

    pub fn set_show_lockout_failures(&mut self, r#show_lockout_failures: bool) {
        self.r#show_lockout_failures = Some(r#show_lockout_failures);
    }

    pub fn with_show_lockout_failures(mut self, r#show_lockout_failures: bool) -> Self {
        self.r#show_lockout_failures = Some(r#show_lockout_failures);
        self
    }

    pub fn r#show_lockout_failures(&self) -> Option<&bool> {
        self.r#show_lockout_failures.as_ref().map(|x| x.borrow())
    }

    pub fn reset_show_lockout_failures(&mut self) {
        self.r#show_lockout_failures = None;
    }

    pub fn set_user_lockout_notification_channels(&mut self, r#user_lockout_notification_channels: Vec<String>) {
        self.r#user_lockout_notification_channels = Some(r#user_lockout_notification_channels);
    }

    pub fn with_user_lockout_notification_channels(mut self, r#user_lockout_notification_channels: Vec<String>) -> Self {
        self.r#user_lockout_notification_channels = Some(r#user_lockout_notification_channels);
        self
    }

    pub fn r#user_lockout_notification_channels(&self) -> Option<&Vec<String>> {
        self.r#user_lockout_notification_channels.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_lockout_notification_channels(&mut self) {
        self.r#user_lockout_notification_channels = None;
    }
}
