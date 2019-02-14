#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Session {
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "amr", skip_serializing_if = "Option::is_none")]
    r#amr: Option<Vec<SessionAuthenticationMethod>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    r#created_at: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    r#expires_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "idp", skip_serializing_if = "Option::is_none")]
    r#idp: Option<SessionIdentityProvider>,
    #[serde(rename = "lastFactorVerification", skip_serializing_if = "Option::is_none")]
    r#last_factor_verification: Option<String>,
    #[serde(rename = "lastPasswordVerification", skip_serializing_if = "Option::is_none")]
    r#last_password_verification: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    r#login: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<SessionStatus>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    r#user_id: Option<String>,
}

impl r#Session {
    pub fn new(
    ) -> Self {
        Self {
          r#links: None,
          r#amr: None,
          r#created_at: None,
          r#expires_at: None,
          r#id: None,
          r#idp: None,
          r#last_factor_verification: None,
          r#last_password_verification: None,
          r#login: None,
          r#status: None,
          r#user_id: None,
        }
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

    pub fn set_amr(&mut self, r#amr: Vec<SessionAuthenticationMethod>) {
        self.r#amr = Some(r#amr);
    }

    pub fn with_amr(mut self, r#amr: Vec<SessionAuthenticationMethod>) -> Self {
        self.r#amr = Some(r#amr);
        self
    }

    pub fn r#amr(&self) -> Option<&Vec<SessionAuthenticationMethod>> {
        self.r#amr.as_ref().map(|x| x.borrow())
    }

    pub fn reset_amr(&mut self) {
        self.r#amr = None;
    }

    pub fn set_created_at(&mut self, r#created_at: String) {
        self.r#created_at = Some(r#created_at);
    }

    pub fn with_created_at(mut self, r#created_at: String) -> Self {
        self.r#created_at = Some(r#created_at);
        self
    }

    pub fn r#created_at(&self) -> Option<&str> {
        self.r#created_at.as_ref().map(|x| x.borrow())
    }

    pub fn reset_created_at(&mut self) {
        self.r#created_at = None;
    }

    pub fn set_expires_at(&mut self, r#expires_at: String) {
        self.r#expires_at = Some(r#expires_at);
    }

    pub fn with_expires_at(mut self, r#expires_at: String) -> Self {
        self.r#expires_at = Some(r#expires_at);
        self
    }

    pub fn r#expires_at(&self) -> Option<&str> {
        self.r#expires_at.as_ref().map(|x| x.borrow())
    }

    pub fn reset_expires_at(&mut self) {
        self.r#expires_at = None;
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

    pub fn set_idp(&mut self, r#idp: SessionIdentityProvider) {
        self.r#idp = Some(r#idp);
    }

    pub fn with_idp(mut self, r#idp: SessionIdentityProvider) -> Self {
        self.r#idp = Some(r#idp);
        self
    }

    pub fn r#idp(&self) -> Option<&SessionIdentityProvider> {
        self.r#idp.as_ref().map(|x| x.borrow())
    }

    pub fn reset_idp(&mut self) {
        self.r#idp = None;
    }

    pub fn set_last_factor_verification(&mut self, r#last_factor_verification: String) {
        self.r#last_factor_verification = Some(r#last_factor_verification);
    }

    pub fn with_last_factor_verification(mut self, r#last_factor_verification: String) -> Self {
        self.r#last_factor_verification = Some(r#last_factor_verification);
        self
    }

    pub fn r#last_factor_verification(&self) -> Option<&str> {
        self.r#last_factor_verification.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_factor_verification(&mut self) {
        self.r#last_factor_verification = None;
    }

    pub fn set_last_password_verification(&mut self, r#last_password_verification: String) {
        self.r#last_password_verification = Some(r#last_password_verification);
    }

    pub fn with_last_password_verification(mut self, r#last_password_verification: String) -> Self {
        self.r#last_password_verification = Some(r#last_password_verification);
        self
    }

    pub fn r#last_password_verification(&self) -> Option<&str> {
        self.r#last_password_verification.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_password_verification(&mut self) {
        self.r#last_password_verification = None;
    }

    pub fn set_login(&mut self, r#login: String) {
        self.r#login = Some(r#login);
    }

    pub fn with_login(mut self, r#login: String) -> Self {
        self.r#login = Some(r#login);
        self
    }

    pub fn r#login(&self) -> Option<&str> {
        self.r#login.as_ref().map(|x| x.borrow())
    }

    pub fn reset_login(&mut self) {
        self.r#login = None;
    }

    pub fn set_status(&mut self, r#status: SessionStatus) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: SessionStatus) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&SessionStatus> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
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
}
