#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#WsFederationApplicationSettingsApplication {
    #[serde(rename = "attributeStatements", skip_serializing_if = "Option::is_none")]
    r#attribute_statements: Option<String>,
    #[serde(rename = "audienceRestriction", skip_serializing_if = "Option::is_none")]
    r#audience_restriction: Option<String>,
    #[serde(rename = "authnContextClassRef", skip_serializing_if = "Option::is_none")]
    r#authn_context_class_ref: Option<String>,
    #[serde(rename = "groupFilter", skip_serializing_if = "Option::is_none")]
    r#group_filter: Option<String>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    r#group_name: Option<String>,
    #[serde(rename = "groupValueFormat", skip_serializing_if = "Option::is_none")]
    r#group_value_format: Option<String>,
    #[serde(rename = "nameIDFormat", skip_serializing_if = "Option::is_none")]
    r#name_id_format: Option<String>,
    #[serde(rename = "realm", skip_serializing_if = "Option::is_none")]
    r#realm: Option<String>,
    #[serde(rename = "siteURL", skip_serializing_if = "Option::is_none")]
    r#site_url: Option<String>,
    #[serde(rename = "usernameAttribute", skip_serializing_if = "Option::is_none")]
    r#username_attribute: Option<String>,
    #[serde(rename = "wReplyOverride", skip_serializing_if = "Option::is_none")]
    r#w_reply_override: Option<bool>,
    #[serde(rename = "wReplyURL", skip_serializing_if = "Option::is_none")]
    r#w_reply_url: Option<String>,
}

impl r#WsFederationApplicationSettingsApplication {
    pub fn new(
    ) -> Self {
        Self {
          r#attribute_statements: None,
          r#audience_restriction: None,
          r#authn_context_class_ref: None,
          r#group_filter: None,
          r#group_name: None,
          r#group_value_format: None,
          r#name_id_format: None,
          r#realm: None,
          r#site_url: None,
          r#username_attribute: None,
          r#w_reply_override: None,
          r#w_reply_url: None,
        }
    }

    pub fn set_attribute_statements(&mut self, r#attribute_statements: String) {
        self.r#attribute_statements = Some(r#attribute_statements);
    }

    pub fn with_attribute_statements(mut self, r#attribute_statements: String) -> Self {
        self.r#attribute_statements = Some(r#attribute_statements);
        self
    }

    pub fn r#attribute_statements(&self) -> Option<&str> {
        self.r#attribute_statements.as_ref().map(|x| x.borrow())
    }

    pub fn reset_attribute_statements(&mut self) {
        self.r#attribute_statements = None;
    }

    pub fn set_audience_restriction(&mut self, r#audience_restriction: String) {
        self.r#audience_restriction = Some(r#audience_restriction);
    }

    pub fn with_audience_restriction(mut self, r#audience_restriction: String) -> Self {
        self.r#audience_restriction = Some(r#audience_restriction);
        self
    }

    pub fn r#audience_restriction(&self) -> Option<&str> {
        self.r#audience_restriction.as_ref().map(|x| x.borrow())
    }

    pub fn reset_audience_restriction(&mut self) {
        self.r#audience_restriction = None;
    }

    pub fn set_authn_context_class_ref(&mut self, r#authn_context_class_ref: String) {
        self.r#authn_context_class_ref = Some(r#authn_context_class_ref);
    }

    pub fn with_authn_context_class_ref(mut self, r#authn_context_class_ref: String) -> Self {
        self.r#authn_context_class_ref = Some(r#authn_context_class_ref);
        self
    }

    pub fn r#authn_context_class_ref(&self) -> Option<&str> {
        self.r#authn_context_class_ref.as_ref().map(|x| x.borrow())
    }

    pub fn reset_authn_context_class_ref(&mut self) {
        self.r#authn_context_class_ref = None;
    }

    pub fn set_group_filter(&mut self, r#group_filter: String) {
        self.r#group_filter = Some(r#group_filter);
    }

    pub fn with_group_filter(mut self, r#group_filter: String) -> Self {
        self.r#group_filter = Some(r#group_filter);
        self
    }

    pub fn r#group_filter(&self) -> Option<&str> {
        self.r#group_filter.as_ref().map(|x| x.borrow())
    }

    pub fn reset_group_filter(&mut self) {
        self.r#group_filter = None;
    }

    pub fn set_group_name(&mut self, r#group_name: String) {
        self.r#group_name = Some(r#group_name);
    }

    pub fn with_group_name(mut self, r#group_name: String) -> Self {
        self.r#group_name = Some(r#group_name);
        self
    }

    pub fn r#group_name(&self) -> Option<&str> {
        self.r#group_name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_group_name(&mut self) {
        self.r#group_name = None;
    }

    pub fn set_group_value_format(&mut self, r#group_value_format: String) {
        self.r#group_value_format = Some(r#group_value_format);
    }

    pub fn with_group_value_format(mut self, r#group_value_format: String) -> Self {
        self.r#group_value_format = Some(r#group_value_format);
        self
    }

    pub fn r#group_value_format(&self) -> Option<&str> {
        self.r#group_value_format.as_ref().map(|x| x.borrow())
    }

    pub fn reset_group_value_format(&mut self) {
        self.r#group_value_format = None;
    }

    pub fn set_name_id_format(&mut self, r#name_id_format: String) {
        self.r#name_id_format = Some(r#name_id_format);
    }

    pub fn with_name_id_format(mut self, r#name_id_format: String) -> Self {
        self.r#name_id_format = Some(r#name_id_format);
        self
    }

    pub fn r#name_id_format(&self) -> Option<&str> {
        self.r#name_id_format.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name_id_format(&mut self) {
        self.r#name_id_format = None;
    }

    pub fn set_realm(&mut self, r#realm: String) {
        self.r#realm = Some(r#realm);
    }

    pub fn with_realm(mut self, r#realm: String) -> Self {
        self.r#realm = Some(r#realm);
        self
    }

    pub fn r#realm(&self) -> Option<&str> {
        self.r#realm.as_ref().map(|x| x.borrow())
    }

    pub fn reset_realm(&mut self) {
        self.r#realm = None;
    }

    pub fn set_site_url(&mut self, r#site_url: String) {
        self.r#site_url = Some(r#site_url);
    }

    pub fn with_site_url(mut self, r#site_url: String) -> Self {
        self.r#site_url = Some(r#site_url);
        self
    }

    pub fn r#site_url(&self) -> Option<&str> {
        self.r#site_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_site_url(&mut self) {
        self.r#site_url = None;
    }

    pub fn set_username_attribute(&mut self, r#username_attribute: String) {
        self.r#username_attribute = Some(r#username_attribute);
    }

    pub fn with_username_attribute(mut self, r#username_attribute: String) -> Self {
        self.r#username_attribute = Some(r#username_attribute);
        self
    }

    pub fn r#username_attribute(&self) -> Option<&str> {
        self.r#username_attribute.as_ref().map(|x| x.borrow())
    }

    pub fn reset_username_attribute(&mut self) {
        self.r#username_attribute = None;
    }

    pub fn set_w_reply_override(&mut self, r#w_reply_override: bool) {
        self.r#w_reply_override = Some(r#w_reply_override);
    }

    pub fn with_w_reply_override(mut self, r#w_reply_override: bool) -> Self {
        self.r#w_reply_override = Some(r#w_reply_override);
        self
    }

    pub fn r#w_reply_override(&self) -> Option<&bool> {
        self.r#w_reply_override.as_ref().map(|x| x.borrow())
    }

    pub fn reset_w_reply_override(&mut self) {
        self.r#w_reply_override = None;
    }

    pub fn set_w_reply_url(&mut self, r#w_reply_url: String) {
        self.r#w_reply_url = Some(r#w_reply_url);
    }

    pub fn with_w_reply_url(mut self, r#w_reply_url: String) -> Self {
        self.r#w_reply_url = Some(r#w_reply_url);
        self
    }

    pub fn r#w_reply_url(&self) -> Option<&str> {
        self.r#w_reply_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_w_reply_url(&mut self) {
        self.r#w_reply_url = None;
    }
}
