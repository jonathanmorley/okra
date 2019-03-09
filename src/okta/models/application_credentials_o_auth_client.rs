#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationCredentialsOAuthClient {
    #[serde(rename = "autoKeyRotation", skip_serializing_if = "Option::is_none")]
    r#auto_key_rotation: Option<bool>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    r#client_id: Option<String>,
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    r#client_secret: Option<String>,
    #[serde(rename = "token_endpoint_auth_method", skip_serializing_if = "Option::is_none")]
    r#token_endpoint_auth_method: Option<OAuthEndpointAuthenticationMethod>,
}

impl r#ApplicationCredentialsOAuthClient {
    pub fn new(
    ) -> Self {
        Self {
          r#auto_key_rotation: None,
          r#client_id: None,
          r#client_secret: None,
          r#token_endpoint_auth_method: None,
        }
    }

    pub fn set_auto_key_rotation(&mut self, r#auto_key_rotation: bool) {
        self.r#auto_key_rotation = Some(r#auto_key_rotation);
    }

    pub fn with_auto_key_rotation(mut self, r#auto_key_rotation: bool) -> Self {
        self.r#auto_key_rotation = Some(r#auto_key_rotation);
        self
    }

    pub fn r#auto_key_rotation(&self) -> Option<&bool> {
        self.r#auto_key_rotation.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auto_key_rotation(&mut self) {
        self.r#auto_key_rotation = None;
    }

    pub fn set_client_id(&mut self, r#client_id: String) {
        self.r#client_id = Some(r#client_id);
    }

    pub fn with_client_id(mut self, r#client_id: String) -> Self {
        self.r#client_id = Some(r#client_id);
        self
    }

    pub fn r#client_id(&self) -> Option<&str> {
        self.r#client_id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_client_id(&mut self) {
        self.r#client_id = None;
    }

    pub fn set_client_secret(&mut self, r#client_secret: String) {
        self.r#client_secret = Some(r#client_secret);
    }

    pub fn with_client_secret(mut self, r#client_secret: String) -> Self {
        self.r#client_secret = Some(r#client_secret);
        self
    }

    pub fn r#client_secret(&self) -> Option<&str> {
        self.r#client_secret.as_ref().map(|x| x.borrow())
    }

    pub fn reset_client_secret(&mut self) {
        self.r#client_secret = None;
    }

    pub fn set_token_endpoint_auth_method(&mut self, r#token_endpoint_auth_method: OAuthEndpointAuthenticationMethod) {
        self.r#token_endpoint_auth_method = Some(r#token_endpoint_auth_method);
    }

    pub fn with_token_endpoint_auth_method(mut self, r#token_endpoint_auth_method: OAuthEndpointAuthenticationMethod) -> Self {
        self.r#token_endpoint_auth_method = Some(r#token_endpoint_auth_method);
        self
    }

    pub fn r#token_endpoint_auth_method(&self) -> Option<&OAuthEndpointAuthenticationMethod> {
        self.r#token_endpoint_auth_method.as_ref().map(|x| x.borrow())
    }

    pub fn reset_token_endpoint_auth_method(&mut self) {
        self.r#token_endpoint_auth_method = None;
    }
}
