#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PolicyPeopleCondition {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    r#groups: Option<GroupCondition>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    r#users: Option<UserCondition>,
}

impl r#PolicyPeopleCondition {
    pub fn new(
    ) -> Self {
        Self {
          r#groups: None,
          r#users: None,
        }
    }

    pub fn set_groups(&mut self, r#groups: GroupCondition) {
        self.r#groups = Some(r#groups);
    }

    pub fn with_groups(mut self, r#groups: GroupCondition) -> Self {
        self.r#groups = Some(r#groups);
        self
    }

    pub fn r#groups(&self) -> Option<&GroupCondition> {
        self.r#groups.as_ref().map(|x| x.borrow())
    }

    pub fn reset_groups(&mut self) {
        self.r#groups = None;
    }

    pub fn set_users(&mut self, r#users: UserCondition) {
        self.r#users = Some(r#users);
    }

    pub fn with_users(mut self, r#users: UserCondition) -> Self {
        self.r#users = Some(r#users);
        self
    }

    pub fn r#users(&self) -> Option<&UserCondition> {
        self.r#users.as_ref().map(|x| x.borrow())
    }

    pub fn reset_users(&mut self) {
        self.r#users = None;
    }
}
