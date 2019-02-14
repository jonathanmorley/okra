#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationAccessibility {
    #[serde(rename = "errorRedirectUrl", skip_serializing_if = "Option::is_none")]
    r#error_redirect_url: Option<String>,
    #[serde(rename = "loginRedirectUrl", skip_serializing_if = "Option::is_none")]
    r#login_redirect_url: Option<String>,
    #[serde(rename = "selfService", skip_serializing_if = "Option::is_none")]
    r#self_service: Option<bool>,
}

impl r#ApplicationAccessibility {
    pub fn new(
    ) -> Self {
        Self {
          r#error_redirect_url: None,
          r#login_redirect_url: None,
          r#self_service: None,
        }
    }

    pub fn set_error_redirect_url(&mut self, r#error_redirect_url: String) {
        self.r#error_redirect_url = Some(r#error_redirect_url);
    }

    pub fn with_error_redirect_url(mut self, r#error_redirect_url: String) -> Self {
        self.r#error_redirect_url = Some(r#error_redirect_url);
        self
    }

    pub fn r#error_redirect_url(&self) -> Option<&str> {
        self.r#error_redirect_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_error_redirect_url(&mut self) {
        self.r#error_redirect_url = None;
    }

    pub fn set_login_redirect_url(&mut self, r#login_redirect_url: String) {
        self.r#login_redirect_url = Some(r#login_redirect_url);
    }

    pub fn with_login_redirect_url(mut self, r#login_redirect_url: String) -> Self {
        self.r#login_redirect_url = Some(r#login_redirect_url);
        self
    }

    pub fn r#login_redirect_url(&self) -> Option<&str> {
        self.r#login_redirect_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login_redirect_url(&mut self) {
        self.r#login_redirect_url = None;
    }

    pub fn set_self_service(&mut self, r#self_service: bool) {
        self.r#self_service = Some(r#self_service);
    }

    pub fn with_self_service(mut self, r#self_service: bool) -> Self {
        self.r#self_service = Some(r#self_service);
        self
    }

    pub fn r#self_service(&self) -> Option<&bool> {
        self.r#self_service.as_ref().map(|x| x.borrow())
    }

    pub fn reset_self_service(&mut self) {
        self.r#self_service = None;
    }
}
