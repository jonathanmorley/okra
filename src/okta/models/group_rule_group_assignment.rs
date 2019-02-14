#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRuleGroupAssignment {
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    r#group_ids: Option<Vec<String>>,
}

impl r#GroupRuleGroupAssignment {
    pub fn new(
    ) -> Self {
        Self {
          r#group_ids: None,
        }
    }

    pub fn set_group_ids(&mut self, r#group_ids: Vec<String>) {
        self.r#group_ids = Some(r#group_ids);
    }

    pub fn with_group_ids(mut self, r#group_ids: Vec<String>) -> Self {
        self.r#group_ids = Some(r#group_ids);
        self
    }

    pub fn r#group_ids(&self) -> Option<&Vec<String>> {
        self.r#group_ids.as_ref().map(|x| x.borrow())
    }

    pub fn reset_group_ids(&mut self) {
        self.r#group_ids = None;
    }
}
