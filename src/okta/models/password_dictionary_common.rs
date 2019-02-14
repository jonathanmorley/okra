#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordDictionaryCommon {
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    r#exclude: Option<bool>,
}

impl r#PasswordDictionaryCommon {
    pub fn new(
    ) -> Self {
        Self {
          r#exclude: None,
        }
    }

    pub fn set_exclude(&mut self, r#exclude: bool) {
        self.r#exclude = Some(r#exclude);
    }

    pub fn with_exclude(mut self, r#exclude: bool) -> Self {
        self.r#exclude = Some(r#exclude);
        self
    }

    pub fn r#exclude(&self) -> Option<&bool> {
        self.r#exclude.as_ref().map(|x| x.borrow())
    }

    pub fn reset_exclude(&mut self) {
        self.r#exclude = None;
    }
}
