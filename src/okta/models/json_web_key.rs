#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#JsonWebKey {
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    r#links: Option<Value>,
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    r#alg: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    r#e: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    r#expires_at: Option<String>,
    #[serde(rename = "key_ops", skip_serializing_if = "Option::is_none")]
    r#key_ops: Option<Vec<String>>,
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    r#kid: Option<String>,
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    r#kty: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    r#n: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<String>,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    r#use: Option<String>,
    #[serde(rename = "x5c", skip_serializing_if = "Option::is_none")]
    r#x5c: Option<Vec<String>>,
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    r#x5t: Option<String>,
    #[serde(rename = "x5t#S256", skip_serializing_if = "Option::is_none")]
    r#x5t_s256: Option<String>,
    #[serde(rename = "x5u", skip_serializing_if = "Option::is_none")]
    r#x5u: Option<String>,
}

impl r#JsonWebKey {
    pub fn new(
    ) -> Self {
        Self {
          r#links: None,
          r#alg: None,
          r#created: None,
          r#e: None,
          r#expires_at: None,
          r#key_ops: None,
          r#kid: None,
          r#kty: None,
          r#last_updated: None,
          r#n: None,
          r#status: None,
          r#use: None,
          r#x5c: None,
          r#x5t: None,
          r#x5t_s256: None,
          r#x5u: None,
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

    pub fn set_alg(&mut self, r#alg: String) {
        self.r#alg = Some(r#alg);
    }

    pub fn with_alg(mut self, r#alg: String) -> Self {
        self.r#alg = Some(r#alg);
        self
    }

    pub fn r#alg(&self) -> Option<&str> {
        self.r#alg.as_ref().map(|x| x.borrow())
    }

    pub fn reset_alg(&mut self) {
        self.r#alg = None;
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

    pub fn set_e(&mut self, r#e: String) {
        self.r#e = Some(r#e);
    }

    pub fn with_e(mut self, r#e: String) -> Self {
        self.r#e = Some(r#e);
        self
    }

    pub fn r#e(&self) -> Option<&str> {
        self.r#e.as_ref().map(|x| x.borrow())
    }

    pub fn reset_e(&mut self) {
        self.r#e = None;
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

    pub fn set_key_ops(&mut self, r#key_ops: Vec<String>) {
        self.r#key_ops = Some(r#key_ops);
    }

    pub fn with_key_ops(mut self, r#key_ops: Vec<String>) -> Self {
        self.r#key_ops = Some(r#key_ops);
        self
    }

    pub fn r#key_ops(&self) -> Option<&Vec<String>> {
        self.r#key_ops.as_ref().map(|x| x.borrow())
    }

    pub fn reset_key_ops(&mut self) {
        self.r#key_ops = None;
    }

    pub fn set_kid(&mut self, r#kid: String) {
        self.r#kid = Some(r#kid);
    }

    pub fn with_kid(mut self, r#kid: String) -> Self {
        self.r#kid = Some(r#kid);
        self
    }

    pub fn r#kid(&self) -> Option<&str> {
        self.r#kid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_kid(&mut self) {
        self.r#kid = None;
    }

    pub fn set_kty(&mut self, r#kty: String) {
        self.r#kty = Some(r#kty);
    }

    pub fn with_kty(mut self, r#kty: String) -> Self {
        self.r#kty = Some(r#kty);
        self
    }

    pub fn r#kty(&self) -> Option<&str> {
        self.r#kty.as_ref().map(|x| x.borrow())
    }

    pub fn reset_kty(&mut self) {
        self.r#kty = None;
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

    pub fn set_n(&mut self, r#n: String) {
        self.r#n = Some(r#n);
    }

    pub fn with_n(mut self, r#n: String) -> Self {
        self.r#n = Some(r#n);
        self
    }

    pub fn r#n(&self) -> Option<&str> {
        self.r#n.as_ref().map(|x| x.borrow())
    }

    pub fn reset_n(&mut self) {
        self.r#n = None;
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

    pub fn set_use(&mut self, r#use: String) {
        self.r#use = Some(r#use);
    }

    pub fn with_use(mut self, r#use: String) -> Self {
        self.r#use = Some(r#use);
        self
    }

    pub fn r#use(&self) -> Option<&str> {
        self.r#use.as_ref().map(|x| x.borrow())
    }

    pub fn reset_use(&mut self) {
        self.r#use = None;
    }

    pub fn set_x5c(&mut self, r#x5c: Vec<String>) {
        self.r#x5c = Some(r#x5c);
    }

    pub fn with_x5c(mut self, r#x5c: Vec<String>) -> Self {
        self.r#x5c = Some(r#x5c);
        self
    }

    pub fn r#x5c(&self) -> Option<&Vec<String>> {
        self.r#x5c.as_ref().map(|x| x.borrow())
    }

    pub fn reset_x5c(&mut self) {
        self.r#x5c = None;
    }

    pub fn set_x5t(&mut self, r#x5t: String) {
        self.r#x5t = Some(r#x5t);
    }

    pub fn with_x5t(mut self, r#x5t: String) -> Self {
        self.r#x5t = Some(r#x5t);
        self
    }

    pub fn r#x5t(&self) -> Option<&str> {
        self.r#x5t.as_ref().map(|x| x.borrow())
    }

    pub fn reset_x5t(&mut self) {
        self.r#x5t = None;
    }

    pub fn set_x5t_s256(&mut self, r#x5t_s256: String) {
        self.r#x5t_s256 = Some(r#x5t_s256);
    }

    pub fn with_x5t_s256(mut self, r#x5t_s256: String) -> Self {
        self.r#x5t_s256 = Some(r#x5t_s256);
        self
    }

    pub fn r#x5t_s256(&self) -> Option<&str> {
        self.r#x5t_s256.as_ref().map(|x| x.borrow())
    }

    pub fn reset_x5t_s256(&mut self) {
        self.r#x5t_s256 = None;
    }

    pub fn set_x5u(&mut self, r#x5u: String) {
        self.r#x5u = Some(r#x5u);
    }

    pub fn with_x5u(mut self, r#x5u: String) -> Self {
        self.r#x5u = Some(r#x5u);
        self
    }

    pub fn r#x5u(&self) -> Option<&str> {
        self.r#x5u.as_ref().map(|x| x.borrow())
    }

    pub fn reset_x5u(&mut self) {
        self.r#x5u = None;
    }
}
