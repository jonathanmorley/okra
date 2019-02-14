#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Factor {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    r#device: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    r#device_type: Option<String>,
    #[serde(rename = "factorType", skip_serializing_if = "Option::is_none")]
    r#factor_type: Option<FactorType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "mfaStateTokenId", skip_serializing_if = "Option::is_none")]
    r#mfa_state_token_id: Option<String>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    r#profile: Option<FactorProfile>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    r#provider: Option<FactorProvider>,
    #[serde(rename = "rechallengeExistingFactor", skip_serializing_if = "Option::is_none")]
    r#rechallenge_existing_factor: Option<bool>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    r#session_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<FactorStatus>,
    #[serde(rename = "tokenLifetimeSeconds", skip_serializing_if = "Option::is_none")]
    r#token_lifetime_seconds: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    r#user_id: Option<String>,
    #[serde(rename = "verify", skip_serializing_if = "Option::is_none")]
    r#verify: Option<VerifyFactorRequest>,
}

impl r#Factor {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#links: None,
          r#device: None,
          r#device_type: None,
          r#factor_type: None,
          r#id: None,
          r#mfa_state_token_id: None,
          r#profile: None,
          r#provider: None,
          r#rechallenge_existing_factor: None,
          r#session_id: None,
          r#status: None,
          r#token_lifetime_seconds: None,
          r#user_id: None,
          r#verify: None,
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

    pub fn set_device(&mut self, r#device: String) {
        self.r#device = Some(r#device);
    }

    pub fn with_device(mut self, r#device: String) -> Self {
        self.r#device = Some(r#device);
        self
    }

    pub fn r#device(&self) -> Option<&str> {
        self.r#device.as_ref().map(|x| x.borrow())
    }

    pub fn reset_device(&mut self) {
        self.r#device = None;
    }

    pub fn set_device_type(&mut self, r#device_type: String) {
        self.r#device_type = Some(r#device_type);
    }

    pub fn with_device_type(mut self, r#device_type: String) -> Self {
        self.r#device_type = Some(r#device_type);
        self
    }

    pub fn r#device_type(&self) -> Option<&str> {
        self.r#device_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_device_type(&mut self) {
        self.r#device_type = None;
    }

    pub fn set_factor_type(&mut self, r#factor_type: FactorType) {
        self.r#factor_type = Some(r#factor_type);
    }

    pub fn with_factor_type(mut self, r#factor_type: FactorType) -> Self {
        self.r#factor_type = Some(r#factor_type);
        self
    }

    pub fn r#factor_type(&self) -> Option<&FactorType> {
        self.r#factor_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_factor_type(&mut self) {
        self.r#factor_type = None;
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

    pub fn set_mfa_state_token_id(&mut self, r#mfa_state_token_id: String) {
        self.r#mfa_state_token_id = Some(r#mfa_state_token_id);
    }

    pub fn with_mfa_state_token_id(mut self, r#mfa_state_token_id: String) -> Self {
        self.r#mfa_state_token_id = Some(r#mfa_state_token_id);
        self
    }

    pub fn r#mfa_state_token_id(&self) -> Option<&str> {
        self.r#mfa_state_token_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_mfa_state_token_id(&mut self) {
        self.r#mfa_state_token_id = None;
    }

    pub fn set_profile(&mut self, r#profile: FactorProfile) {
        self.r#profile = Some(r#profile);
    }

    pub fn with_profile(mut self, r#profile: FactorProfile) -> Self {
        self.r#profile = Some(r#profile);
        self
    }

    pub fn r#profile(&self) -> Option<&FactorProfile> {
        self.r#profile.as_ref().map(|x| x.borrow())
    }

    pub fn reset_profile(&mut self) {
        self.r#profile = None;
    }

    pub fn set_provider(&mut self, r#provider: FactorProvider) {
        self.r#provider = Some(r#provider);
    }

    pub fn with_provider(mut self, r#provider: FactorProvider) -> Self {
        self.r#provider = Some(r#provider);
        self
    }

    pub fn r#provider(&self) -> Option<&FactorProvider> {
        self.r#provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_provider(&mut self) {
        self.r#provider = None;
    }

    pub fn set_rechallenge_existing_factor(&mut self, r#rechallenge_existing_factor: bool) {
        self.r#rechallenge_existing_factor = Some(r#rechallenge_existing_factor);
    }

    pub fn with_rechallenge_existing_factor(mut self, r#rechallenge_existing_factor: bool) -> Self {
        self.r#rechallenge_existing_factor = Some(r#rechallenge_existing_factor);
        self
    }

    pub fn r#rechallenge_existing_factor(&self) -> Option<&bool> {
        self.r#rechallenge_existing_factor.as_ref().map(|x| x.borrow())
    }

    pub fn reset_rechallenge_existing_factor(&mut self) {
        self.r#rechallenge_existing_factor = None;
    }

    pub fn set_session_id(&mut self, r#session_id: String) {
        self.r#session_id = Some(r#session_id);
    }

    pub fn with_session_id(mut self, r#session_id: String) -> Self {
        self.r#session_id = Some(r#session_id);
        self
    }

    pub fn r#session_id(&self) -> Option<&str> {
        self.r#session_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_session_id(&mut self) {
        self.r#session_id = None;
    }

    pub fn set_status(&mut self, r#status: FactorStatus) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: FactorStatus) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&FactorStatus> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }

    pub fn set_token_lifetime_seconds(&mut self, r#token_lifetime_seconds: i32) {
        self.r#token_lifetime_seconds = Some(r#token_lifetime_seconds);
    }

    pub fn with_token_lifetime_seconds(mut self, r#token_lifetime_seconds: i32) -> Self {
        self.r#token_lifetime_seconds = Some(r#token_lifetime_seconds);
        self
    }

    pub fn r#token_lifetime_seconds(&self) -> Option<&i32> {
        self.r#token_lifetime_seconds.as_ref().map(|x| x.borrow())
    }

    pub fn reset_token_lifetime_seconds(&mut self) {
        self.r#token_lifetime_seconds = None;
    }

    pub fn set_user_id(&mut self, r#user_id: String) {
        self.r#user_id = Some(r#user_id);
    }

    pub fn with_user_id(mut self, r#user_id: String) -> Self {
        self.r#user_id = Some(r#user_id);
        self
    }

    pub fn r#user_id(&self) -> Option<&str> {
        self.r#user_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_id(&mut self) {
        self.r#user_id = None;
    }

    pub fn set_verify(&mut self, r#verify: VerifyFactorRequest) {
        self.r#verify = Some(r#verify);
    }

    pub fn with_verify(mut self, r#verify: VerifyFactorRequest) -> Self {
        self.r#verify = Some(r#verify);
        self
    }

    pub fn r#verify(&self) -> Option<&VerifyFactorRequest> {
        self.r#verify.as_ref().map(|x| x.borrow())
    }

    pub fn reset_verify(&mut self) {
        self.r#verify = None;
    }
}
