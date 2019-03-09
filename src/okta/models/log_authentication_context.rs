#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogAuthenticationContext {
    #[serde(rename = "authenticationProvider", skip_serializing_if = "Option::is_none")]
    r#authentication_provider: Option<LogAuthenticationProvider>,
    #[serde(rename = "authenticationStep", skip_serializing_if = "Option::is_none")]
    r#authentication_step: Option<i32>,
    #[serde(rename = "credentialProvider", skip_serializing_if = "Option::is_none")]
    r#credential_provider: Option<Vec<LogCredentialProvider>>,
    #[serde(rename = "credentialType", skip_serializing_if = "Option::is_none")]
    r#credential_type: Option<Vec<LogCredentialType>>,
    #[serde(rename = "externalSessionId", skip_serializing_if = "Option::is_none")]
    r#external_session_id: Option<String>,
    #[serde(rename = "interface", skip_serializing_if = "Option::is_none")]
    r#interface: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    r#issuer: Option<LogIssuer>,
}

impl r#LogAuthenticationContext {
    pub fn new(
    ) -> Self {
        Self {
          r#authentication_provider: None,
          r#authentication_step: None,
          r#credential_provider: None,
          r#credential_type: None,
          r#external_session_id: None,
          r#interface: None,
          r#issuer: None,
        }
    }

    pub fn set_authentication_provider(&mut self, r#authentication_provider: LogAuthenticationProvider) {
        self.r#authentication_provider = Some(r#authentication_provider);
    }

    pub fn with_authentication_provider(mut self, r#authentication_provider: LogAuthenticationProvider) -> Self {
        self.r#authentication_provider = Some(r#authentication_provider);
        self
    }

    pub fn r#authentication_provider(&self) -> Option<&LogAuthenticationProvider> {
        self.r#authentication_provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_authentication_provider(&mut self) {
        self.r#authentication_provider = None;
    }

    pub fn set_authentication_step(&mut self, r#authentication_step: i32) {
        self.r#authentication_step = Some(r#authentication_step);
    }

    pub fn with_authentication_step(mut self, r#authentication_step: i32) -> Self {
        self.r#authentication_step = Some(r#authentication_step);
        self
    }

    pub fn r#authentication_step(&self) -> Option<&i32> {
        self.r#authentication_step.as_ref().map(|x| x.borrow())
    }

    pub fn reset_authentication_step(&mut self) {
        self.r#authentication_step = None;
    }

    pub fn set_credential_provider(&mut self, r#credential_provider: Vec<LogCredentialProvider>) {
        self.r#credential_provider = Some(r#credential_provider);
    }

    pub fn with_credential_provider(mut self, r#credential_provider: Vec<LogCredentialProvider>) -> Self {
        self.r#credential_provider = Some(r#credential_provider);
        self
    }

    pub fn r#credential_provider(&self) -> Option<&Vec<LogCredentialProvider>> {
        self.r#credential_provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credential_provider(&mut self) {
        self.r#credential_provider = None;
    }

    pub fn set_credential_type(&mut self, r#credential_type: Vec<LogCredentialType>) {
        self.r#credential_type = Some(r#credential_type);
    }

    pub fn with_credential_type(mut self, r#credential_type: Vec<LogCredentialType>) -> Self {
        self.r#credential_type = Some(r#credential_type);
        self
    }

    pub fn r#credential_type(&self) -> Option<&Vec<LogCredentialType>> {
        self.r#credential_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_credential_type(&mut self) {
        self.r#credential_type = None;
    }

    pub fn set_external_session_id(&mut self, r#external_session_id: String) {
        self.r#external_session_id = Some(r#external_session_id);
    }

    pub fn with_external_session_id(mut self, r#external_session_id: String) -> Self {
        self.r#external_session_id = Some(r#external_session_id);
        self
    }

    pub fn r#external_session_id(&self) -> Option<&str> {
        self.r#external_session_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_external_session_id(&mut self) {
        self.r#external_session_id = None;
    }

    pub fn set_interface(&mut self, r#interface: String) {
        self.r#interface = Some(r#interface);
    }

    pub fn with_interface(mut self, r#interface: String) -> Self {
        self.r#interface = Some(r#interface);
        self
    }

    pub fn r#interface(&self) -> Option<&str> {
        self.r#interface.as_ref().map(|x| x.borrow())
    }

    pub fn reset_interface(&mut self) {
        self.r#interface = None;
    }

    pub fn set_issuer(&mut self, r#issuer: LogIssuer) {
        self.r#issuer = Some(r#issuer);
    }

    pub fn with_issuer(mut self, r#issuer: LogIssuer) -> Self {
        self.r#issuer = Some(r#issuer);
        self
    }

    pub fn r#issuer(&self) -> Option<&LogIssuer> {
        self.r#issuer.as_ref().map(|x| x.borrow())
    }

    pub fn reset_issuer(&mut self) {
        self.r#issuer = None;
    }
}
