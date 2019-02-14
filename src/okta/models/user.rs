#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#User {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "activated", skip_serializing_if = "Option::is_none")]
    r#activated: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    r#credentials: Option<UserCredentials>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "lastLogin", skip_serializing_if = "Option::is_none")]
    r#last_login: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "passwordChanged", skip_serializing_if = "Option::is_none")]
    r#password_changed: Option<String>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<UserProfile>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<UserStatus>,
    #[serde(rename = "statusChanged", skip_serializing_if = "Option::is_none")]
    r#status_changed: Option<String>,
    #[serde(rename = "transitioningToStatus", skip_serializing_if = "Option::is_none")]
    r#transitioning_to_status: Option<UserStatus>,
}

impl r#User {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#activated: None,
          r#created: None,
          r#credentials: None,
          r#id: None,
          r#last_login: None,
          r#last_updated: None,
          r#password_changed: None,
          r#profile: None,
          r#status: None,
          r#status_changed: None,
          r#transitioning_to_status: None,
        }
    }

    pub fn set_embedded(&mut self, r#embedded: Value) {
        self.r#embedded = Some(r#embedded);
    }

    pub fn with_embedded(mut self, r#embedded: Value) -> Self {
        self.r#embedded = Some(r#embedded);
        self
    }

    pub fn r#embedded(&self) -> Option<&Value> {
        self.r#embedded.as_ref().map(|x| x.borrow())
    }

    pub fn reset_embedded(&mut self) {
        self.r#embedded = None;
    }

    pub fn set_links(&mut self, r#links: Value) {
        self.r#links = Some(r#links);
    }

    pub fn with_links(mut self, r#links: Value) -> Self {
        self.r#links = Some(r#links);
        self
    }

    pub fn r#links(&self) -> Option<&Value> {
        self.r#links.as_ref().map(|x| x.borrow())
    }

    pub fn reset_links(&mut self) {
        self.r#links = None;
    }

    pub fn set_activated(&mut self, r#activated: String) {
        self.r#activated = Some(r#activated);
    }

    pub fn with_activated(mut self, r#activated: String) -> Self {
        self.r#activated = Some(r#activated);
        self
    }

    pub fn r#activated(&self) -> Option<&str> {
        self.r#activated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_activated(&mut self) {
        self.r#activated = None;
    }

    pub fn set_created(&mut self, r#created: String) {
        self.r#created = Some(r#created);
    }

    pub fn with_created(mut self, r#created: String) -> Self {
        self.r#created = Some(r#created);
        self
    }

    pub fn r#created(&self) -> Option<&str> {
        self.r#created.as_ref().map(|x| x.borrow())
    }

    pub fn reset_created(&mut self) {
        self.r#created = None;
    }

    pub fn set_credentials(&mut self, r#credentials: UserCredentials) {
        self.r#credentials = Some(r#credentials);
    }

    pub fn with_credentials(mut self, r#credentials: UserCredentials) -> Self {
        self.r#credentials = Some(r#credentials);
        self
    }

    pub fn r#credentials(&self) -> Option<&UserCredentials> {
        self.r#credentials.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credentials(&mut self) {
        self.r#credentials = None;
    }

    pub fn set_id(&mut self, r#id: String) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: String) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn r#id(&self) -> Option<&str> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
    }

    pub fn set_last_login(&mut self, r#last_login: String) {
        self.r#last_login = Some(r#last_login);
    }

    pub fn with_last_login(mut self, r#last_login: String) -> Self {
        self.r#last_login = Some(r#last_login);
        self
    }

    pub fn r#last_login(&self) -> Option<&str> {
        self.r#last_login.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_login(&mut self) {
        self.r#last_login = None;
    }

    pub fn set_last_updated(&mut self, r#last_updated: String) {
        self.r#last_updated = Some(r#last_updated);
    }

    pub fn with_last_updated(mut self, r#last_updated: String) -> Self {
        self.r#last_updated = Some(r#last_updated);
        self
    }

    pub fn r#last_updated(&self) -> Option<&str> {
        self.r#last_updated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_updated(&mut self) {
        self.r#last_updated = None;
    }

    pub fn set_password_changed(&mut self, r#password_changed: String) {
        self.r#password_changed = Some(r#password_changed);
    }

    pub fn with_password_changed(mut self, r#password_changed: String) -> Self {
        self.r#password_changed = Some(r#password_changed);
        self
    }

    pub fn r#password_changed(&self) -> Option<&str> {
        self.r#password_changed.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password_changed(&mut self) {
        self.r#password_changed = None;
    }

    pub fn set_profile(&mut self, r#profile: UserProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: UserProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&UserProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }

    pub fn set_status(&mut self, r#status: UserStatus) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: UserStatus) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&UserStatus> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }

    pub fn set_status_changed(&mut self, r#status_changed: String) {
        self.r#status_changed = Some(r#status_changed);
    }

    pub fn with_status_changed(mut self, r#status_changed: String) -> Self {
        self.r#status_changed = Some(r#status_changed);
        self
    }

    pub fn r#status_changed(&self) -> Option<&str> {
        self.r#status_changed.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status_changed(&mut self) {
        self.r#status_changed = None;
    }

    pub fn set_transitioning_to_status(&mut self, r#transitioning_to_status: UserStatus) {
        self.r#transitioning_to_status = Some(r#transitioning_to_status);
    }

    pub fn with_transitioning_to_status(mut self, r#transitioning_to_status: UserStatus) -> Self {
        self.r#transitioning_to_status = Some(r#transitioning_to_status);
        self
    }

    pub fn r#transitioning_to_status(&self) -> Option<&UserStatus> {
        self.r#transitioning_to_status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_transitioning_to_status(&mut self) {
        self.r#transitioning_to_status = None;
    }
}
