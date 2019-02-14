#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRuleAction {
    #[serde(rename = "assignUserToGroups", skip_serializing_if = "Option::is_none")]
    r#assign_user_to_groups: Option<GroupRuleGroupAssignment>,
}

impl r#GroupRuleAction {
    pub fn new(
    ) -> Self {
        Self {
          r#assign_user_to_groups: None,
        }
    }

    pub fn set_assign_user_to_groups(&mut self, r#assign_user_to_groups: GroupRuleGroupAssignment) {
        self.r#assign_user_to_groups = Some(r#assign_user_to_groups);
    }

    pub fn with_assign_user_to_groups(mut self, r#assign_user_to_groups: GroupRuleGroupAssignment) -> Self {
        self.r#assign_user_to_groups = Some(r#assign_user_to_groups);
        self
    }

    pub fn r#assign_user_to_groups(&self) -> Option<&GroupRuleGroupAssignment> {
        self.r#assign_user_to_groups.as_ref().map(|x| x.borrow())
    }

    pub fn reset_assign_user_to_groups(&mut self) {
        self.r#assign_user_to_groups = None;
    }
}
