#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#RecoveryQuestionCredential {
    #[serde(rename = "answer", skip_serializing_if = "Option::is_none")]
    r#answer: Option<String>,
    #[serde(rename = "question", skip_serializing_if = "Option::is_none")]
    r#question: Option<String>,
}

impl r#RecoveryQuestionCredential {
    pub fn new(
    ) -> Self {
        Self {
          r#answer: None,
          r#question: None,
        }
    }

    pub fn set_answer(&mut self, r#answer: String) {
        self.r#answer = Some(r#answer);
    }

    pub fn with_answer(mut self, r#answer: String) -> Self {
        self.r#answer = Some(r#answer);
        self
    }

    pub fn r#answer(&self) -> Option<&str> {
        self.r#answer.as_ref().map(|x| x.borrow())
    }

    pub fn reset_answer(&mut self) {
        self.r#answer = None;
    }

    pub fn set_question(&mut self, r#question: String) {
        self.r#question = Some(r#question);
    }

    pub fn with_question(mut self, r#question: String) -> Self {
        self.r#question = Some(r#question);
        self
    }

    pub fn r#question(&self) -> Option<&str> {
        self.r#question.as_ref().map(|x| x.borrow())
    }

    pub fn reset_question(&mut self) {
        self.r#question = None;
    }
}
