#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyRuleConditions {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    r#network: Option<PolicyNetworkCondition>,
    #[serde(rename = "people", skip_serializing_if = "Option::is_none")]
    r#people: Option<PolicyPeopleCondition>,
}

impl r#PasswordPolicyRuleConditions {
    pub fn new(
    ) -> Self {
        Self {
          r#network: None,
          r#people: None,
        }
    }

    pub fn set_network(&mut self, r#network: PolicyNetworkCondition) {
        self.r#network = Some(r#network);
    }

    pub fn with_network(mut self, r#network: PolicyNetworkCondition) -> Self {
        self.r#network = Some(r#network);
        self
    }

    pub fn r#network(&self) -> Option<&PolicyNetworkCondition> {
        self.r#network.as_ref().map(|x| x.borrow())
    }

    pub fn reset_network(&mut self) {
        self.r#network = None;
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
