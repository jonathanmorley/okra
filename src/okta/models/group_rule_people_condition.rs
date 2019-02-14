#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRulePeopleCondition {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    r#groups: Option<GroupRuleGroupCondition>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    r#users: Option<GroupRuleUserCondition>,
}

impl r#GroupRulePeopleCondition {
    pub fn new(
    ) -> Self {
        Self {
          r#groups: None,
          r#users: None,
        }
    }

    pub fn set_groups(&mut self, r#groups: GroupRuleGroupCondition) {
        self.r#groups = Some(r#groups);
    }

    pub fn with_groups(mut self, r#groups: GroupRuleGroupCondition) -> Self {
        self.r#groups = Some(r#groups);
        self
    }

    pub fn r#groups(&self) -> Option<&GroupRuleGroupCondition> {
        self.r#groups.as_ref().map(|x| x.borrow())
    }

    pub fn reset_groups(&mut self) {
        self.r#groups = None;
    }

    pub fn set_users(&mut self, r#users: GroupRuleUserCondition) {
        self.r#users = Some(r#users);
    }

    pub fn with_users(mut self, r#users: GroupRuleUserCondition) -> Self {
        self.r#users = Some(r#users);
        self
    }

    pub fn r#users(&self) -> Option<&GroupRuleUserCondition> {
        self.r#users.as_ref().map(|x| x.borrow())
    }

    pub fn reset_users(&mut self) {
        self.r#users = None;
    }
}
