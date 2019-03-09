#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordDictionary {
    #[serde(rename = "common", skip_serializing_if = "Option::is_none")]
    r#common: Option<PasswordDictionaryCommon>,
}

impl r#PasswordDictionary {
    pub fn new(
    ) -> Self {
        Self {
          r#common: None,
        }
    }

    pub fn set_common(&mut self, r#common: PasswordDictionaryCommon) {
        self.r#common = Some(r#common);
    }

    pub fn with_common(mut self, r#common: PasswordDictionaryCommon) -> Self {
        self.r#common = Some(r#common);
        self
    }

    pub fn r#common(&self) -> Option<&PasswordDictionaryCommon> {
        self.r#common.as_ref().map(|x| x.borrow())
    }

    pub fn reset_common(&mut self) {
        self.r#common = None;
    }
}
