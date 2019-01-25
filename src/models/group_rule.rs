/* 
 * Okta API
 *
 * Allows customers to easily access the Okta API
 *
 * OpenAPI spec version: 1.9.0
 * Contact: devex-public@okta.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupRule {
  #[serde(rename = "_embedded")]
  _embedded: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "actions")]
  actions: Option<crate::models::GroupRuleAction>,
  #[serde(rename = "allGroupsValid")]
  all_groups_valid: Option<bool>,
  #[serde(rename = "conditions")]
  conditions: Option<crate::models::GroupRuleConditions>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "lastUpdated")]
  last_updated: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "status")]
  status: Option<crate::models::GroupRuleStatus>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl GroupRule {
  pub fn new() -> GroupRule {
    GroupRule {
      _embedded: None,
      actions: None,
      all_groups_valid: None,
      conditions: None,
      created: None,
      id: None,
      last_updated: None,
      name: None,
      status: None,
      _type: None
    }
  }

  pub fn set_embedded(&mut self, _embedded: ::std::collections::HashMap<String, Value>) {
    self._embedded = Some(_embedded);
  }

  pub fn with_embedded(mut self, _embedded: ::std::collections::HashMap<String, Value>) -> GroupRule {
    self._embedded = Some(_embedded);
    self
  }

  pub fn _embedded(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self._embedded.as_ref()
  }

  pub fn reset_embedded(&mut self) {
    self._embedded = None;
  }

  pub fn set_actions(&mut self, actions: crate::models::GroupRuleAction) {
    self.actions = Some(actions);
  }

  pub fn with_actions(mut self, actions: crate::models::GroupRuleAction) -> GroupRule {
    self.actions = Some(actions);
    self
  }

  pub fn actions(&self) -> Option<&crate::models::GroupRuleAction> {
    self.actions.as_ref()
  }

  pub fn reset_actions(&mut self) {
    self.actions = None;
  }

  pub fn set_all_groups_valid(&mut self, all_groups_valid: bool) {
    self.all_groups_valid = Some(all_groups_valid);
  }

  pub fn with_all_groups_valid(mut self, all_groups_valid: bool) -> GroupRule {
    self.all_groups_valid = Some(all_groups_valid);
    self
  }

  pub fn all_groups_valid(&self) -> Option<&bool> {
    self.all_groups_valid.as_ref()
  }

  pub fn reset_all_groups_valid(&mut self) {
    self.all_groups_valid = None;
  }

  pub fn set_conditions(&mut self, conditions: crate::models::GroupRuleConditions) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: crate::models::GroupRuleConditions) -> GroupRule {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&crate::models::GroupRuleConditions> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> GroupRule {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> GroupRule {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_last_updated(&mut self, last_updated: String) {
    self.last_updated = Some(last_updated);
  }

  pub fn with_last_updated(mut self, last_updated: String) -> GroupRule {
    self.last_updated = Some(last_updated);
    self
  }

  pub fn last_updated(&self) -> Option<&String> {
    self.last_updated.as_ref()
  }

  pub fn reset_last_updated(&mut self) {
    self.last_updated = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GroupRule {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_status(&mut self, status: crate::models::GroupRuleStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: crate::models::GroupRuleStatus) -> GroupRule {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&crate::models::GroupRuleStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with_type(mut self, _type: String) -> GroupRule {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset_type(&mut self) {
    self._type = None;
  }

}



