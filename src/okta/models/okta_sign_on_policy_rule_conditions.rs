#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyRuleConditions {
    #[serde(rename = "authContext", skip_serializing_if = "Option::is_none")]
    r#auth_context: Option<PolicyRuleAuthContextCondition>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    r#network: Option<PolicyNetworkCondition>,
    #[serde(rename = "people", skip_serializing_if = "Option::is_none")]
    r#people: Option<PolicyPeopleCondition>,
}

impl r#OktaSignOnPolicyRuleConditions {
    pub fn new(
    ) -> Self {
        Self {
          r#auth_context: None,
          r#network: None,
          r#people: None,
        }
    }

    pub fn set_auth_context(&mut self, r#auth_context: PolicyRuleAuthContextCondition) {
        self.r#auth_context = Some(r#auth_context);
    }

    pub fn with_auth_context(mut self, r#auth_context: PolicyRuleAuthContextCondition) -> Self {
        self.r#auth_context = Some(r#auth_context);
        self
    }

    pub fn r#auth_context(&self) -> Option<&PolicyRuleAuthContextCondition> {
        self.r#auth_context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auth_context(&mut self) {
        self.r#auth_context = None;
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
