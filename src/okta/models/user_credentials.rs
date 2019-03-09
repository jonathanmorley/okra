#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#UserCredentials {
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    r#emails: Option<Vec<EmailAddress>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    r#password: Option<PasswordCredential>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    r#provider: Option<AuthenticationProvider>,
    #[serde(rename = "recovery_question", skip_serializing_if = "Option::is_none")]
    r#recovery_question: Option<RecoveryQuestionCredential>,
}

impl r#UserCredentials {
    pub fn new(
    ) -> Self {
        Self {
          r#emails: None,
          r#password: None,
          r#provider: None,
          r#recovery_question: None,
        }
    }

    pub fn set_emails(&mut self, r#emails: Vec<EmailAddress>) {
        self.r#emails = Some(r#emails);
    }

    pub fn with_emails(mut self, r#emails: Vec<EmailAddress>) -> Self {
        self.r#emails = Some(r#emails);
        self
    }

    pub fn r#emails(&self) -> Option<&Vec<EmailAddress>> {
        self.r#emails.as_ref().map(|x| x.borrow())
    }

    pub fn reset_emails(&mut self) {
        self.r#emails = None;
    }

    pub fn set_password(&mut self, r#password: PasswordCredential) {
        self.r#password = Some(r#password);
    }

    pub fn with_password(mut self, r#password: PasswordCredential) -> Self {
        self.r#password = Some(r#password);
        self
    }

    pub fn r#password(&self) -> Option<&PasswordCredential> {
        self.r#password.as_ref().map(|x| x.borrow())
    }

    pub fn reset_password(&mut self) {
        self.r#password = None;
    }

    pub fn set_provider(&mut self, r#provider: AuthenticationProvider) {
        self.r#provider = Some(r#provider);
    }

    pub fn with_provider(mut self, r#provider: AuthenticationProvider) -> Self {
        self.r#provider = Some(r#provider);
        self
    }

    pub fn r#provider(&self) -> Option<&AuthenticationProvider> {
        self.r#provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_provider(&mut self) {
        self.r#provider = None;
    }

    pub fn set_recovery_question(&mut self, r#recovery_question: RecoveryQuestionCredential) {
        self.r#recovery_question = Some(r#recovery_question);
    }

    pub fn with_recovery_question(mut self, r#recovery_question: RecoveryQuestionCredential) -> Self {
        self.r#recovery_question = Some(r#recovery_question);
        self
    }

    pub fn r#recovery_question(&self) -> Option<&RecoveryQuestionCredential> {
        self.r#recovery_question.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recovery_question(&mut self) {
        self.r#recovery_question = None;
    }
}
