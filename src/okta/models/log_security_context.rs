#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogSecurityContext {
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    r#as_number: Option<i32>,
    #[serde(rename = "asOrg", skip_serializing_if = "Option::is_none")]
    r#as_org: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    r#domain: Option<String>,
    #[serde(rename = "isProxy", skip_serializing_if = "Option::is_none")]
    r#is_proxy: Option<bool>,
    #[serde(rename = "isp", skip_serializing_if = "Option::is_none")]
    r#isp: Option<String>,
}

impl r#LogSecurityContext {
    pub fn new(
    ) -> Self {
        Self {
          r#as_number: None,
          r#as_org: None,
          r#domain: None,
          r#is_proxy: None,
          r#isp: None,
        }
    }

    pub fn set_as_number(&mut self, r#as_number: i32) {
        self.r#as_number = Some(r#as_number);
    }

    pub fn with_as_number(mut self, r#as_number: i32) -> Self {
        self.r#as_number = Some(r#as_number);
        self
    }

    pub fn r#as_number(&self) -> Option<&i32> {
        self.r#as_number.as_ref().map(|x| x.borrow())
    }

    pub fn reset_as_number(&mut self) {
        self.r#as_number = None;
    }

    pub fn set_as_org(&mut self, r#as_org: String) {
        self.r#as_org = Some(r#as_org);
    }

    pub fn with_as_org(mut self, r#as_org: String) -> Self {
        self.r#as_org = Some(r#as_org);
        self
    }

    pub fn r#as_org(&self) -> Option<&str> {
        self.r#as_org.as_ref().map(|x| x.borrow())
    }

    pub fn reset_as_org(&mut self) {
        self.r#as_org = None;
    }

    pub fn set_domain(&mut self, r#domain: String) {
        self.r#domain = Some(r#domain);
    }

    pub fn with_domain(mut self, r#domain: String) -> Self {
        self.r#domain = Some(r#domain);
        self
    }

    pub fn r#domain(&self) -> Option<&str> {
        self.r#domain.as_ref().map(|x| x.borrow())
    }

    pub fn reset_domain(&mut self) {
        self.r#domain = None;
    }

    pub fn set_is_proxy(&mut self, r#is_proxy: bool) {
        self.r#is_proxy = Some(r#is_proxy);
    }

    pub fn with_is_proxy(mut self, r#is_proxy: bool) -> Self {
        self.r#is_proxy = Some(r#is_proxy);
        self
    }

    pub fn r#is_proxy(&self) -> Option<&bool> {
        self.r#is_proxy.as_ref().map(|x| x.borrow())
    }

    pub fn reset_is_proxy(&mut self) {
        self.r#is_proxy = None;
    }

    pub fn set_isp(&mut self, r#isp: String) {
        self.r#isp = Some(r#isp);
    }

    pub fn with_isp(mut self, r#isp: String) -> Self {
        self.r#isp = Some(r#isp);
        self
    }

    pub fn r#isp(&self) -> Option<&str> {
        self.r#isp.as_ref().map(|x| x.borrow())
    }

    pub fn reset_isp(&mut self) {
        self.r#isp = None;
    }
}
