#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRecoveryFactors {
    #[serde(rename = "okta_call", skip_serializing_if = "Option::is_none")]
    r#okta_call: Option<PasswordPolicyRecoveryFactorSettings>,
    #[serde(rename = "okta_email", skip_serializing_if = "Option::is_none")]
    r#okta_email: Option<PasswordPolicyRecoveryEmail>,
    #[serde(rename = "okta_sms", skip_serializing_if = "Option::is_none")]
    r#okta_sms: Option<PasswordPolicyRecoveryFactorSettings>,
    #[serde(rename = "recovery_question", skip_serializing_if = "Option::is_none")]
    r#recovery_question: Option<PasswordPolicyRecoveryQuestion>,
}

impl r#PasswordPolicyRecoveryFactors {
    pub fn new(
    ) -> Self {
        Self {
          r#okta_call: None,
          r#okta_email: None,
          r#okta_sms: None,
          r#recovery_question: None,
        }
    }

    pub fn set_okta_call(&mut self, r#okta_call: PasswordPolicyRecoveryFactorSettings) {
        self.r#okta_call = Some(r#okta_call);
    }

    pub fn with_okta_call(mut self, r#okta_call: PasswordPolicyRecoveryFactorSettings) -> Self {
        self.r#okta_call = Some(r#okta_call);
        self
    }

    pub fn r#okta_call(&self) -> Option<&PasswordPolicyRecoveryFactorSettings> {
        self.r#okta_call.as_ref().map(|x| x.borrow())
    }

    pub fn reset_okta_call(&mut self) {
        self.r#okta_call = None;
    }

    pub fn set_okta_email(&mut self, r#okta_email: PasswordPolicyRecoveryEmail) {
        self.r#okta_email = Some(r#okta_email);
    }

    pub fn with_okta_email(mut self, r#okta_email: PasswordPolicyRecoveryEmail) -> Self {
        self.r#okta_email = Some(r#okta_email);
        self
    }

    pub fn r#okta_email(&self) -> Option<&PasswordPolicyRecoveryEmail> {
        self.r#okta_email.as_ref().map(|x| x.borrow())
    }

    pub fn reset_okta_email(&mut self) {
        self.r#okta_email = None;
    }

    pub fn set_okta_sms(&mut self, r#okta_sms: PasswordPolicyRecoveryFactorSettings) {
        self.r#okta_sms = Some(r#okta_sms);
    }

    pub fn with_okta_sms(mut self, r#okta_sms: PasswordPolicyRecoveryFactorSettings) -> Self {
        self.r#okta_sms = Some(r#okta_sms);
        self
    }

    pub fn r#okta_sms(&self) -> Option<&PasswordPolicyRecoveryFactorSettings> {
        self.r#okta_sms.as_ref().map(|x| x.borrow())
    }

    pub fn reset_okta_sms(&mut self) {
        self.r#okta_sms = None;
    }

    pub fn set_recovery_question(&mut self, r#recovery_question: PasswordPolicyRecoveryQuestion) {
        self.r#recovery_question = Some(r#recovery_question);
    }

    pub fn with_recovery_question(mut self, r#recovery_question: PasswordPolicyRecoveryQuestion) -> Self {
        self.r#recovery_question = Some(r#recovery_question);
        self
    }

    pub fn r#recovery_question(&self) -> Option<&PasswordPolicyRecoveryQuestion> {
        self.r#recovery_question.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recovery_question(&mut self) {
        self.r#recovery_question = None;
    }
}
