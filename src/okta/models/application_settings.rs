#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationSettings {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    r#app: Option<ApplicationSettingsApplication>,
    #[serde(rename = "implicitAssignment", skip_serializing_if = "Option::is_none")]
    r#implicit_assignment: Option<bool>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    r#notifications: Option<ApplicationSettingsNotifications>,
}

impl r#ApplicationSettings {
    pub fn new(
    ) -> Self {
        Self {
          r#app: None,
          r#implicit_assignment: None,
          r#notifications: None,
        }
    }

    pub fn set_app(&mut self, r#app: ApplicationSettingsApplication) {
        self.r#app = Some(r#app);
    }

    pub fn with_app(mut self, r#app: ApplicationSettingsApplication) -> Self {
        self.r#app = Some(r#app);
        self
    }

    pub fn r#app(&self) -> Option<&ApplicationSettingsApplication> {
        self.r#app.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app(&mut self) {
        self.r#app = None;
    }

    pub fn set_implicit_assignment(&mut self, r#implicit_assignment: bool) {
        self.r#implicit_assignment = Some(r#implicit_assignment);
    }

    pub fn with_implicit_assignment(mut self, r#implicit_assignment: bool) -> Self {
        self.r#implicit_assignment = Some(r#implicit_assignment);
        self
    }

    pub fn r#implicit_assignment(&self) -> Option<&bool> {
        self.r#implicit_assignment.as_ref().map(|x| x.borrow())
    }

    pub fn reset_implicit_assignment(&mut self) {
        self.r#implicit_assignment = None;
    }

    pub fn set_notifications(&mut self, r#notifications: ApplicationSettingsNotifications) {
        self.r#notifications = Some(r#notifications);
    }

    pub fn with_notifications(mut self, r#notifications: ApplicationSettingsNotifications) -> Self {
        self.r#notifications = Some(r#notifications);
        self
    }

    pub fn r#notifications(&self) -> Option<&ApplicationSettingsNotifications> {
        self.r#notifications.as_ref().map(|x| x.borrow())
    }

    pub fn reset_notifications(&mut self) {
        self.r#notifications = None;
    }
}
