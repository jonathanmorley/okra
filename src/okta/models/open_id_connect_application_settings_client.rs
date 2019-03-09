#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OpenIdConnectApplicationSettingsClient {
    #[serde(rename = "application_type", skip_serializing_if = "Option::is_none")]
    r#application_type: Option<OpenIdConnectApplicationType>,
    #[serde(rename = "client_uri", skip_serializing_if = "Option::is_none")]
    r#client_uri: Option<String>,
    #[serde(rename = "consent_method", skip_serializing_if = "Option::is_none")]
    r#consent_method: Option<OpenIdConnectApplicationConsentMethod>,
    #[serde(rename = "grant_types", skip_serializing_if = "Option::is_none")]
    r#grant_types: Option<Vec<OAuthGrantType>>,
    #[serde(rename = "logo_uri", skip_serializing_if = "Option::is_none")]
    r#logo_uri: Option<String>,
    #[serde(rename = "policy_uri", skip_serializing_if = "Option::is_none")]
    r#policy_uri: Option<String>,
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    r#redirect_uris: Option<Vec<String>>,
    #[serde(rename = "response_types", skip_serializing_if = "Option::is_none")]
    r#response_types: Option<Vec<OAuthResponseType>>,
    #[serde(rename = "tos_uri", skip_serializing_if = "Option::is_none")]
    r#tos_uri: Option<String>,
}

impl r#OpenIdConnectApplicationSettingsClient {
    pub fn new(
    ) -> Self {
        Self {
          r#application_type: None,
          r#client_uri: None,
          r#consent_method: None,
          r#grant_types: None,
          r#logo_uri: None,
          r#policy_uri: None,
          r#redirect_uris: None,
          r#response_types: None,
          r#tos_uri: None,
        }
    }

    pub fn set_application_type(&mut self, r#application_type: OpenIdConnectApplicationType) {
        self.r#application_type = Some(r#application_type);
    }

    pub fn with_application_type(mut self, r#application_type: OpenIdConnectApplicationType) -> Self {
        self.r#application_type = Some(r#application_type);
        self
    }

    pub fn r#application_type(&self) -> Option<&OpenIdConnectApplicationType> {
        self.r#application_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_application_type(&mut self) {
        self.r#application_type = None;
    }

    pub fn set_client_uri(&mut self, r#client_uri: String) {
        self.r#client_uri = Some(r#client_uri);
    }

    pub fn with_client_uri(mut self, r#client_uri: String) -> Self {
        self.r#client_uri = Some(r#client_uri);
        self
    }

    pub fn r#client_uri(&self) -> Option<&str> {
        self.r#client_uri.as_ref().map(|x| x.borrow())
    }

    pub fn reset_client_uri(&mut self) {
        self.r#client_uri = None;
    }

    pub fn set_consent_method(&mut self, r#consent_method: OpenIdConnectApplicationConsentMethod) {
        self.r#consent_method = Some(r#consent_method);
    }

    pub fn with_consent_method(mut self, r#consent_method: OpenIdConnectApplicationConsentMethod) -> Self {
        self.r#consent_method = Some(r#consent_method);
        self
    }

    pub fn r#consent_method(&self) -> Option<&OpenIdConnectApplicationConsentMethod> {
        self.r#consent_method.as_ref().map(|x| x.borrow())
    }

    pub fn reset_consent_method(&mut self) {
        self.r#consent_method = None;
    }

    pub fn set_grant_types(&mut self, r#grant_types: Vec<OAuthGrantType>) {
        self.r#grant_types = Some(r#grant_types);
    }

    pub fn with_grant_types(mut self, r#grant_types: Vec<OAuthGrantType>) -> Self {
        self.r#grant_types = Some(r#grant_types);
        self
    }

    pub fn r#grant_types(&self) -> Option<&Vec<OAuthGrantType>> {
        self.r#grant_types.as_ref().map(|x| x.borrow())
    }

    pub fn reset_grant_types(&mut self) {
        self.r#grant_types = None;
    }

    pub fn set_logo_uri(&mut self, r#logo_uri: String) {
        self.r#logo_uri = Some(r#logo_uri);
    }

    pub fn with_logo_uri(mut self, r#logo_uri: String) -> Self {
        self.r#logo_uri = Some(r#logo_uri);
        self
    }

    pub fn r#logo_uri(&self) -> Option<&str> {
        self.r#logo_uri.as_ref().map(|x| x.borrow())
    }

    pub fn reset_logo_uri(&mut self) {
        self.r#logo_uri = None;
    }

    pub fn set_policy_uri(&mut self, r#policy_uri: String) {
        self.r#policy_uri = Some(r#policy_uri);
    }

    pub fn with_policy_uri(mut self, r#policy_uri: String) -> Self {
        self.r#policy_uri = Some(r#policy_uri);
        self
    }

    pub fn r#policy_uri(&self) -> Option<&str> {
        self.r#policy_uri.as_ref().map(|x| x.borrow())
    }

    pub fn reset_policy_uri(&mut self) {
        self.r#policy_uri = None;
    }

    pub fn set_redirect_uris(&mut self, r#redirect_uris: Vec<String>) {
        self.r#redirect_uris = Some(r#redirect_uris);
    }

    pub fn with_redirect_uris(mut self, r#redirect_uris: Vec<String>) -> Self {
        self.r#redirect_uris = Some(r#redirect_uris);
        self
    }

    pub fn r#redirect_uris(&self) -> Option<&Vec<String>> {
        self.r#redirect_uris.as_ref().map(|x| x.borrow())
    }

    pub fn reset_redirect_uris(&mut self) {
        self.r#redirect_uris = None;
    }

    pub fn set_response_types(&mut self, r#response_types: Vec<OAuthResponseType>) {
        self.r#response_types = Some(r#response_types);
    }

    pub fn with_response_types(mut self, r#response_types: Vec<OAuthResponseType>) -> Self {
        self.r#response_types = Some(r#response_types);
        self
    }

    pub fn r#response_types(&self) -> Option<&Vec<OAuthResponseType>> {
        self.r#response_types.as_ref().map(|x| x.borrow())
    }

    pub fn reset_response_types(&mut self) {
        self.r#response_types = None;
    }

    pub fn set_tos_uri(&mut self, r#tos_uri: String) {
        self.r#tos_uri = Some(r#tos_uri);
    }

    pub fn with_tos_uri(mut self, r#tos_uri: String) -> Self {
        self.r#tos_uri = Some(r#tos_uri);
        self
    }

    pub fn r#tos_uri(&self) -> Option<&str> {
        self.r#tos_uri.as_ref().map(|x| x.borrow())
    }

    pub fn reset_tos_uri(&mut self) {
        self.r#tos_uri = None;
    }
}
