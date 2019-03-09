#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryQuestionComplexity {
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    r#min_length: Option<i32>,
}

impl r#PasswordPolicyRecoveryQuestionComplexity {
    pub fn new(
    ) -> Self {
        Self {
          r#min_length: None,
        }
    }

    pub fn set_min_length(&mut self, r#min_length: i32) {
        self.r#min_length = Some(r#min_length);
    }

    pub fn with_min_length(mut self, r#min_length: i32) -> Self {
        self.r#min_length = Some(r#min_length);
        self
    }

    pub fn r#min_length(&self) -> Option<&i32> {
        self.r#min_length.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_length(&mut self) {
        self.r#min_length = None;
    }
}
