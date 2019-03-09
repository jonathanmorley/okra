#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationCredentials {
    #[serde(rename = "signing", skip_serializing_if = "Option::is_none")]
    r#signing: Option<ApplicationCredentialsSigning>,
    #[serde(rename = "userNameTemplate", skip_serializing_if = "Option::is_none")]
    r#user_name_template: Option<ApplicationCredentialsUsernameTemplate>,
}

impl r#ApplicationCredentials {
    pub fn new(
    ) -> Self {
        Self {
          r#signing: None,
          r#user_name_template: None,
        }
    }

    pub fn set_signing(&mut self, r#signing: ApplicationCredentialsSigning) {
        self.r#signing = Some(r#signing);
    }

    pub fn with_signing(mut self, r#signing: ApplicationCredentialsSigning) -> Self {
        self.r#signing = Some(r#signing);
        self
    }

    pub fn r#signing(&self) -> Option<&ApplicationCredentialsSigning> {
        self.r#signing.as_ref().map(|x| x.borrow())
    }

    pub fn reset_signing(&mut self) {
        self.r#signing = None;
    }

    pub fn set_user_name_template(&mut self, r#user_name_template: ApplicationCredentialsUsernameTemplate) {
        self.r#user_name_template = Some(r#user_name_template);
    }

    pub fn with_user_name_template(mut self, r#user_name_template: ApplicationCredentialsUsernameTemplate) -> Self {
        self.r#user_name_template = Some(r#user_name_template);
        self
    }

    pub fn r#user_name_template(&self) -> Option<&ApplicationCredentialsUsernameTemplate> {
        self.r#user_name_template.as_ref().map(|x| x.borrow())
    }

    pub fn reset_user_name_template(&mut self) {
        self.r#user_name_template = None;
    }
}
