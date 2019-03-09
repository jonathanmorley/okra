#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#GroupRule {
    #[serde(rename = "_embedded", skip_serializing_if = "Option::is_none")]
    r#embedded: Option<Value>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    r#actions: Option<GroupRuleAction>,
    #[serde(rename = "allGroupsValid", skip_serializing_if = "Option::is_none")]
    r#all_groups_valid: Option<bool>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    r#conditions: Option<GroupRuleConditions>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    r#created: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    r#last_updated: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    r#status: Option<GroupRuleStatus>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl r#GroupRule {
    pub fn new(
    ) -> Self {
        Self {
          r#embedded: None,
          r#actions: None,
          r#all_groups_valid: None,
          r#conditions: None,
          r#created: None,
          r#id: None,
          r#last_updated: None,
          r#name: None,
          r#status: None,
          r#type: None,
        }
    }

    pub fn set_embedded(&mut self, r#embedded: Value) {
        self.r#embedded = Some(r#embedded);
    }

    pub fn with_embedded(mut self, r#embedded: Value) -> Self {
        self.r#embedded = Some(r#embedded);
        self
    }

    pub fn r#embedded(&self) -> Option<&Value> {
        self.r#embedded.as_ref().map(|x| x.borrow())
    }

    pub fn reset_embedded(&mut self) {
        self.r#embedded = None;
    }

    pub fn set_actions(&mut self, r#actions: GroupRuleAction) {
        self.r#actions = Some(r#actions);
    }

    pub fn with_actions(mut self, r#actions: GroupRuleAction) -> Self {
        self.r#actions = Some(r#actions);
        self
    }

    pub fn r#actions(&self) -> Option<&GroupRuleAction> {
        self.r#actions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_actions(&mut self) {
        self.r#actions = None;
    }

    pub fn set_all_groups_valid(&mut self, r#all_groups_valid: bool) {
        self.r#all_groups_valid = Some(r#all_groups_valid);
    }

    pub fn with_all_groups_valid(mut self, r#all_groups_valid: bool) -> Self {
        self.r#all_groups_valid = Some(r#all_groups_valid);
        self
    }

    pub fn r#all_groups_valid(&self) -> Option<&bool> {
        self.r#all_groups_valid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_all_groups_valid(&mut self) {
        self.r#all_groups_valid = None;
    }

    pub fn set_conditions(&mut self, r#conditions: GroupRuleConditions) {
        self.r#conditions = Some(r#conditions);
    }

    pub fn with_conditions(mut self, r#conditions: GroupRuleConditions) -> Self {
        self.r#conditions = Some(r#conditions);
        self
    }

    pub fn r#conditions(&self) -> Option<&GroupRuleConditions> {
        self.r#conditions.as_ref().map(|x| x.borrow())
    }

    pub fn reset_conditions(&mut self) {
        self.r#conditions = None;
    }

    pub fn set_created(&mut self, r#created: String) {
        self.r#created = Some(r#created);
    }

    pub fn with_created(mut self, r#created: String) -> Self {
        self.r#created = Some(r#created);
        self
    }

    pub fn r#created(&self) -> Option<&str> {
        self.r#created.as_ref().map(|x| x.borrow())
    }

    pub fn reset_created(&mut self) {
        self.r#created = None;
    }

    pub fn set_id(&mut self, r#id: String) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: String) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn r#id(&self) -> Option<&str> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
    }

    pub fn set_last_updated(&mut self, r#last_updated: String) {
        self.r#last_updated = Some(r#last_updated);
    }

    pub fn with_last_updated(mut self, r#last_updated: String) -> Self {
        self.r#last_updated = Some(r#last_updated);
        self
    }

    pub fn r#last_updated(&self) -> Option<&str> {
        self.r#last_updated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_updated(&mut self) {
        self.r#last_updated = None;
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = Some(r#name);
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = Some(r#name);
        self
    }

    pub fn r#name(&self) -> Option<&str> {
        self.r#name.as_ref().map(|x| x.borrow())
    }

    pub fn reset_name(&mut self) {
        self.r#name = None;
    }

    pub fn set_status(&mut self, r#status: GroupRuleStatus) {
        self.r#status = Some(r#status);
    }

    pub fn with_status(mut self, r#status: GroupRuleStatus) -> Self {
        self.r#status = Some(r#status);
        self
    }

    pub fn r#status(&self) -> Option<&GroupRuleStatus> {
        self.r#status.as_ref().map(|x| x.borrow())
    }

    pub fn reset_status(&mut self) {
        self.r#status = None;
    }

    pub fn set_type(&mut self, r#type: String) {
        self.r#type = Some(r#type);
    }

    pub fn with_type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn r#type(&self) -> Option<&str> {
        self.r#type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_type(&mut self) {
        self.r#type = None;
    }
}
