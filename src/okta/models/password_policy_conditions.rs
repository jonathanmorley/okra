#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyConditions {
    #[serde(rename = "authProvider", skip_serializing_if = "Option::is_none")]
    r#auth_provider: Option<PasswordPolicyAuthenticationProviderCondition>,
    #[serde(rename = "people", skip_serializing_if = "Option::is_none")]
    r#people: Option<PolicyPeopleCondition>,
}

impl r#PasswordPolicyConditions {
    pub fn new(
    ) -> Self {
        Self {
          r#auth_provider: None,
          r#people: None,
        }
    }

    pub fn set_auth_provider(&mut self, r#auth_provider: PasswordPolicyAuthenticationProviderCondition) {
        self.r#auth_provider = Some(r#auth_provider);
    }

    pub fn with_auth_provider(mut self, r#auth_provider: PasswordPolicyAuthenticationProviderCondition) -> Self {
        self.r#auth_provider = Some(r#auth_provider);
        self
    }

    pub fn r#auth_provider(&self) -> Option<&PasswordPolicyAuthenticationProviderCondition> {
        self.r#auth_provider.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auth_provider(&mut self) {
        self.r#auth_provider = None;
    }

    pub fn set_people(&mut self, r#people: PolicyPeopleCondition) {
        self.r#people = Some(r#people);
    }

    pub fn with_people(mut self, r#people: PolicyPeopleCondition) -> Self {
        self.r#people = Some(r#people);
        self
    }

    pub fn r#people(&self) -> Option<&PolicyPeopleCondition> {
        self.r#people.as_ref().map(|x| x.borrow())
    }

    pub fn reset_people(&mut self) {
        self.r#people = None;
    }
}
