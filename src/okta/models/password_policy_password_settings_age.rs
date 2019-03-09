#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyPasswordSettingsAge {
    #[serde(rename = "expireWarnDays", skip_serializing_if = "Option::is_none")]
    r#expire_warn_days: Option<i32>,
    #[serde(rename = "historyCount", skip_serializing_if = "Option::is_none")]
    r#history_count: Option<i32>,
    #[serde(rename = "maxAgeDays", skip_serializing_if = "Option::is_none")]
    r#max_age_days: Option<i32>,
    #[serde(rename = "minAgeMinutes", skip_serializing_if = "Option::is_none")]
    r#min_age_minutes: Option<i32>,
}

impl r#PasswordPolicyPasswordSettingsAge {
    pub fn new(
    ) -> Self {
        Self {
          r#expire_warn_days: None,
          r#history_count: None,
          r#max_age_days: None,
          r#min_age_minutes: None,
        }
    }

    pub fn set_expire_warn_days(&mut self, r#expire_warn_days: i32) {
        self.r#expire_warn_days = Some(r#expire_warn_days);
    }

    pub fn with_expire_warn_days(mut self, r#expire_warn_days: i32) -> Self {
        self.r#expire_warn_days = Some(r#expire_warn_days);
        self
    }

    pub fn r#expire_warn_days(&self) -> Option<&i32> {
        self.r#expire_warn_days.as_ref().map(|x| x.borrow())
    }

    pub fn reset_expire_warn_days(&mut self) {
        self.r#expire_warn_days = None;
    }

    pub fn set_history_count(&mut self, r#history_count: i32) {
        self.r#history_count = Some(r#history_count);
    }

    pub fn with_history_count(mut self, r#history_count: i32) -> Self {
        self.r#history_count = Some(r#history_count);
        self
    }

    pub fn r#history_count(&self) -> Option<&i32> {
        self.r#history_count.as_ref().map(|x| x.borrow())
    }

    pub fn reset_history_count(&mut self) {
        self.r#history_count = None;
    }

    pub fn set_max_age_days(&mut self, r#max_age_days: i32) {
        self.r#max_age_days = Some(r#max_age_days);
    }

    pub fn with_max_age_days(mut self, r#max_age_days: i32) -> Self {
        self.r#max_age_days = Some(r#max_age_days);
        self
    }

    pub fn r#max_age_days(&self) -> Option<&i32> {
        self.r#max_age_days.as_ref().map(|x| x.borrow())
    }

    pub fn reset_max_age_days(&mut self) {
        self.r#max_age_days = None;
    }

    pub fn set_min_age_minutes(&mut self, r#min_age_minutes: i32) {
        self.r#min_age_minutes = Some(r#min_age_minutes);
    }

    pub fn with_min_age_minutes(mut self, r#min_age_minutes: i32) -> Self {
        self.r#min_age_minutes = Some(r#min_age_minutes);
        self
    }

    pub fn r#min_age_minutes(&self) -> Option<&i32> {
        self.r#min_age_minutes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_age_minutes(&mut self) {
        self.r#min_age_minutes = None;
    }
}
