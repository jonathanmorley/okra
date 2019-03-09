#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryQuestionProperties {
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    r#complexity: Option<PasswordPolicyRecoveryQuestionComplexity>,
}

impl r#PasswordPolicyRecoveryQuestionProperties {
    pub fn new(
    ) -> Self {
        Self {
          r#complexity: None,
        }
    }

    pub fn set_complexity(&mut self, r#complexity: PasswordPolicyRecoveryQuestionComplexity) {
        self.r#complexity = Some(r#complexity);
    }

    pub fn with_complexity(mut self, r#complexity: PasswordPolicyRecoveryQuestionComplexity) -> Self {
        self.r#complexity = Some(r#complexity);
        self
    }

    pub fn r#complexity(&self) -> Option<&PasswordPolicyRecoveryQuestionComplexity> {
        self.r#complexity.as_ref().map(|x| x.borrow())
    }

    pub fn reset_complexity(&mut self) {
        self.r#complexity = None;
    }
}
