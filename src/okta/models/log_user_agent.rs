#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogUserAgent {
    #[serde(rename = "browser", skip_serializing_if = "Option::is_none")]
    r#browser: Option<String>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    r#os: Option<String>,
    #[serde(rename = "rawUserAgent", skip_serializing_if = "Option::is_none")]
    r#raw_user_agent: Option<String>,
}

impl r#LogUserAgent {
    pub fn new(
    ) -> Self {
        Self {
          r#browser: None,
          r#os: None,
          r#raw_user_agent: None,
        }
    }

    pub fn set_browser(&mut self, r#browser: String) {
        self.r#browser = Some(r#browser);
    }

    pub fn with_browser(mut self, r#browser: String) -> Self {
        self.r#browser = Some(r#browser);
        self
    }

    pub fn r#browser(&self) -> Option<&str> {
        self.r#browser.as_ref().map(|x| x.borrow())
    }

    pub fn reset_browser(&mut self) {
        self.r#browser = None;
    }

    pub fn set_os(&mut self, r#os: String) {
        self.r#os = Some(r#os);
    }

    pub fn with_os(mut self, r#os: String) -> Self {
        self.r#os = Some(r#os);
        self
    }

    pub fn r#os(&self) -> Option<&str> {
        self.r#os.as_ref().map(|x| x.borrow())
    }

    pub fn reset_os(&mut self) {
        self.r#os = None;
    }

    pub fn set_raw_user_agent(&mut self, r#raw_user_agent: String) {
        self.r#raw_user_agent = Some(r#raw_user_agent);
    }

    pub fn with_raw_user_agent(mut self, r#raw_user_agent: String) -> Self {
        self.r#raw_user_agent = Some(r#raw_user_agent);
        self
    }

    pub fn r#raw_user_agent(&self) -> Option<&str> {
        self.r#raw_user_agent.as_ref().map(|x| x.borrow())
    }

    pub fn reset_raw_user_agent(&mut self) {
        self.r#raw_user_agent = None;
    }
}
