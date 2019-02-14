#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyConditions {
    #[serde(rename = "people", skip_serializing_if = "Option::is_none")]
    r#people: Option<PolicyPeopleCondition>,
}

impl r#OktaSignOnPolicyConditions {
    pub fn new(
    ) -> Self {
        Self {
          r#people: None,
        }
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
