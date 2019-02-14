#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Application {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "accessibility", skip_serializing_if = "Option::is_none")]
    r#accessibility: Option<ApplicationAccessibility>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    r#credentials: Option<ApplicationCredentials>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    r#features: Option<Vec<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    r#label: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "licensing", skip_serializing_if = "Option::is_none")]
    r#licensing: Option<ApplicationLicensing>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<Value>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    r#settings: Option<ApplicationSettings>,
    #[serde(rename = "signOnMode", skip_serializing_if = "Option::is_none")]
    r#sign_on_mode: Option<ApplicationSignOnMode>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<String>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    r#visibility: Option<ApplicationVisibility>,
}

impl r#Application {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#accessibility: None,
          r#created: None,
          r#credentials: None,
          r#features: None,
          r#id: None,
          r#label: None,
          r#last_updated: None,
          r#licensing: None,
          r#name: None,
          r#profile: None,
          r#settings: None,
          r#sign_on_mode: None,
          r#status: None,
          r#visibility: None,
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

    pub fn set_accessibility(&mut self, r#accessibility: ApplicationAccessibility) {
        self.r#accessibility = Some(r#accessibility);
    }

    pub fn with_accessibility(mut self, r#accessibility: ApplicationAccessibility) -> Self {
        self.r#accessibility = Some(r#accessibility);
        self
    }

    pub fn r#accessibility(&self) -> Option<&ApplicationAccessibility> {
        self.r#accessibility.as_ref().map(|x| x.borrow())
    }

    pub fn reset_accessibility(&mut self) {
        self.r#accessibility = None;
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

    pub fn set_credentials(&mut self, r#credentials: ApplicationCredentials) {
        self.r#credentials = Some(r#credentials);
    }

    pub fn with_credentials(mut self, r#credentials: ApplicationCredentials) -> Self {
        self.r#credentials = Some(r#credentials);
        self
    }

    pub fn r#credentials(&self) -> Option<&ApplicationCredentials> {
        self.r#credentials.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credentials(&mut self) {
        self.r#credentials = None;
    }

    pub fn set_features(&mut self, r#features: Vec<String>) {
        self.r#features = Some(r#features);
    }

    pub fn with_features(mut self, r#features: Vec<String>) -> Self {
        self.r#features = Some(r#features);
        self
    }

    pub fn r#features(&self) -> Option<&Vec<String>> {
        self.r#features.as_ref().map(|x| x.borrow())
    }

    pub fn reset_features(&mut self) {
        self.r#features = None;
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

    pub fn set_label(&mut self, r#label: String) {
        self.r#label = Some(r#label);
    }

    pub fn with_label(mut self, r#label: String) -> Self {
        self.r#label = Some(r#label);
        self
    }

    pub fn r#label(&self) -> Option<&str> {
        self.r#label.as_ref().map(|x| x.borrow())
    }

    pub fn reset_label(&mut self) {
        self.r#label = None;
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

    pub fn set_licensing(&mut self, r#licensing: ApplicationLicensing) {
        self.r#licensing = Some(r#licensing);
    }

    pub fn with_licensing(mut self, r#licensing: ApplicationLicensing) -> Self {
        self.r#licensing = Some(r#licensing);
        self
    }

    pub fn r#licensing(&self) -> Option<&ApplicationLicensing> {
        self.r#licensing.as_ref().map(|x| x.borrow())
    }

    pub fn reset_licensing(&mut self) {
        self.r#licensing = None;
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = Some(r#name);
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = Some(r#name);
        self
    }

    pub fn r#name(&self) -> Option<&str> {
        self.r#name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name(&mut self) {
        self.r#name = None;
    }

    pub fn set_profile(&mut self, r#profile: Value) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: Value) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&Value> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }

    pub fn set_settings(&mut self, r#settings: ApplicationSettings) {
        self.r#settings = Some(r#settings);
    }

    pub fn with_settings(mut self, r#settings: ApplicationSettings) -> Self {
        self.r#settings = Some(r#settings);
        self
    }

    pub fn r#settings(&self) -> Option<&ApplicationSettings> {
        self.r#settings.as_ref().map(|x| x.borrow())
    }

    pub fn reset_settings(&mut self) {
        self.r#settings = None;
    }

    pub fn set_sign_on_mode(&mut self, r#sign_on_mode: ApplicationSignOnMode) {
        self.r#sign_on_mode = Some(r#sign_on_mode);
    }

    pub fn with_sign_on_mode(mut self, r#sign_on_mode: ApplicationSignOnMode) -> Self {
        self.r#sign_on_mode = Some(r#sign_on_mode);
        self
    }

    pub fn r#sign_on_mode(&self) -> Option<&ApplicationSignOnMode> {
        self.r#sign_on_mode.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sign_on_mode(&mut self) {
        self.r#sign_on_mode = None;
    }

    pub fn set_status(&mut self, r#status: String) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: String) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&str> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }

    pub fn set_visibility(&mut self, r#visibility: ApplicationVisibility) {
        self.r#visibility = Some(r#visibility);
    }

    pub fn with_visibility(mut self, r#visibility: ApplicationVisibility) -> Self {
        self.r#visibility = Some(r#visibility);
        self
    }

    pub fn r#visibility(&self) -> Option<&ApplicationVisibility> {
        self.r#visibility.as_ref().map(|x| x.borrow())
    }

    pub fn reset_visibility(&mut self) {
        self.r#visibility = None;
    }
}
