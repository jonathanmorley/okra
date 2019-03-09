#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationVisibilityHide {
    #[serde(rename = "iOS", skip_serializing_if = "Option::is_none")]
    r#i_os: Option<bool>,
    #[serde(rename = "web", skip_serializing_if = "Option::is_none")]
    r#web: Option<bool>,
}

impl r#ApplicationVisibilityHide {
    pub fn new(
    ) -> Self {
        Self {
          r#i_os: None,
          r#web: None,
        }
    }

    pub fn set_i_os(&mut self, r#i_os: bool) {
        self.r#i_os = Some(r#i_os);
    }

    pub fn with_i_os(mut self, r#i_os: bool) -> Self {
        self.r#i_os = Some(r#i_os);
        self
    }

    pub fn r#i_os(&self) -> Option<&bool> {
        self.r#i_os.as_ref().map(|x| x.borrow())
    }

    pub fn reset_i_os(&mut self) {
        self.r#i_os = None;
    }

    pub fn set_web(&mut self, r#web: bool) {
        self.r#web = Some(r#web);
    }

    pub fn with_web(mut self, r#web: bool) -> Self {
        self.r#web = Some(r#web);
        self
    }

    pub fn r#web(&self) -> Option<&bool> {
        self.r#web.as_ref().map(|x| x.borrow())
    }

    pub fn reset_web(&mut self) {
        self.r#web = None;
    }
}
