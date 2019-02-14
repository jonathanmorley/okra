#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRuleConditions {
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    r#expression: Option<GroupRuleExpression>,
    #[serde(rename = "people", skip_serializing_if = "Option::is_none")]
    r#people: Option<GroupRulePeopleCondition>,
}

impl r#GroupRuleConditions {
    pub fn new(
    ) -> Self {
        Self {
          r#expression: None,
          r#people: None,
        }
    }

    pub fn set_expression(&mut self, r#expression: GroupRuleExpression) {
        self.r#expression = Some(r#expression);
    }

    pub fn with_expression(mut self, r#expression: GroupRuleExpression) -> Self {
        self.r#expression = Some(r#expression);
        self
    }

    pub fn r#expression(&self) -> Option<&GroupRuleExpression> {
        self.r#expression.as_ref().map(|x| x.borrow())
    }

    pub fn reset_expression(&mut self) {
        self.r#expression = None;
    }

    pub fn set_people(&mut self, r#people: GroupRulePeopleCondition) {
        self.r#people = Some(r#people);
    }

    pub fn with_people(mut self, r#people: GroupRulePeopleCondition) -> Self {
        self.r#people = Some(r#people);
        self
    }

    pub fn r#people(&self) -> Option<&GroupRulePeopleCondition> {
        self.r#people.as_ref().map(|x| x.borrow())
    }

    pub fn reset_people(&mut self) {
        self.r#people = None;
    }
}
