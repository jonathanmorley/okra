#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#AppLink {
    #[serde(rename = "appAssignmentId", skip_serializing_if = "Option::is_none")]
    r#app_assignment_id: Option<String>,
    #[serde(rename = "appInstanceId", skip_serializing_if = "Option::is_none")]
    r#app_instance_id: Option<String>,
    #[serde(rename = "appName", skip_serializing_if = "Option::is_none")]
    r#app_name: Option<String>,
    #[serde(rename = "credentialsSetup", skip_serializing_if = "Option::is_none")]
    r#credentials_setup: Option<bool>,
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    r#hidden: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    r#label: Option<String>,
    #[serde(rename = "linkUrl", skip_serializing_if = "Option::is_none")]
    r#link_url: Option<String>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    r#logo_url: Option<String>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    r#sort_order: Option<i32>,
}

impl r#AppLink {
    pub fn new(
    ) -> Self {
        Self {
          r#app_assignment_id: None,
          r#app_instance_id: None,
          r#app_name: None,
          r#credentials_setup: None,
          r#hidden: None,
          r#id: None,
          r#label: None,
          r#link_url: None,
          r#logo_url: None,
          r#sort_order: None,
        }
    }

    pub fn set_app_assignment_id(&mut self, r#app_assignment_id: String) {
        self.r#app_assignment_id = Some(r#app_assignment_id);
    }

    pub fn with_app_assignment_id(mut self, r#app_assignment_id: String) -> Self {
        self.r#app_assignment_id = Some(r#app_assignment_id);
        self
    }

    pub fn r#app_assignment_id(&self) -> Option<&str> {
        self.r#app_assignment_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app_assignment_id(&mut self) {
        self.r#app_assignment_id = None;
    }

    pub fn set_app_instance_id(&mut self, r#app_instance_id: String) {
        self.r#app_instance_id = Some(r#app_instance_id);
    }

    pub fn with_app_instance_id(mut self, r#app_instance_id: String) -> Self {
        self.r#app_instance_id = Some(r#app_instance_id);
        self
    }

    pub fn r#app_instance_id(&self) -> Option<&str> {
        self.r#app_instance_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app_instance_id(&mut self) {
        self.r#app_instance_id = None;
    }

    pub fn set_app_name(&mut self, r#app_name: String) {
        self.r#app_name = Some(r#app_name);
    }

    pub fn with_app_name(mut self, r#app_name: String) -> Self {
        self.r#app_name = Some(r#app_name);
        self
    }

    pub fn r#app_name(&self) -> Option<&str> {
        self.r#app_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app_name(&mut self) {
        self.r#app_name = None;
    }

    pub fn set_credentials_setup(&mut self, r#credentials_setup: bool) {
        self.r#credentials_setup = Some(r#credentials_setup);
    }

    pub fn with_credentials_setup(mut self, r#credentials_setup: bool) -> Self {
        self.r#credentials_setup = Some(r#credentials_setup);
        self
    }

    pub fn r#credentials_setup(&self) -> Option<&bool> {
        self.r#credentials_setup.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credentials_setup(&mut self) {
        self.r#credentials_setup = None;
    }

    pub fn set_hidden(&mut self, r#hidden: bool) {
        self.r#hidden = Some(r#hidden);
    }

    pub fn with_hidden(mut self, r#hidden: bool) -> Self {
        self.r#hidden = Some(r#hidden);
        self
    }

    pub fn r#hidden(&self) -> Option<&bool> {
        self.r#hidden.as_ref().map(|x| x.borrow())
    }

    pub fn reset_hidden(&mut self) {
        self.r#hidden = None;
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

    pub fn set_link_url(&mut self, r#link_url: String) {
        self.r#link_url = Some(r#link_url);
    }

    pub fn with_link_url(mut self, r#link_url: String) -> Self {
        self.r#link_url = Some(r#link_url);
        self
    }

    pub fn r#link_url(&self) -> Option<&str> {
        self.r#link_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_link_url(&mut self) {
        self.r#link_url = None;
    }

    pub fn set_logo_url(&mut self, r#logo_url: String) {
        self.r#logo_url = Some(r#logo_url);
    }

    pub fn with_logo_url(mut self, r#logo_url: String) -> Self {
        self.r#logo_url = Some(r#logo_url);
        self
    }

    pub fn r#logo_url(&self) -> Option<&str> {
        self.r#logo_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_logo_url(&mut self) {
        self.r#logo_url = None;
    }

    pub fn set_sort_order(&mut self, r#sort_order: i32) {
        self.r#sort_order = Some(r#sort_order);
    }

    pub fn with_sort_order(mut self, r#sort_order: i32) -> Self {
        self.r#sort_order = Some(r#sort_order);
        self
    }

    pub fn r#sort_order(&self) -> Option<&i32> {
        self.r#sort_order.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sort_order(&mut self) {
        self.r#sort_order = None;
    }
}
