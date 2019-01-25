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
pub struct Policy {
  #[serde(rename = "_embedded")]
  _embedded: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "_links")]
  _links: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "lastUpdated")]
  last_updated: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "priority")]
  priority: Option<i32>,
  #[serde(rename = "status")]
  status: Option<String>,
  #[serde(rename = "system")]
  system: Option<bool>,
  #[serde(rename = "type")]
  _type: Option<crate::models::PolicyType>
}

impl Policy {
  pub fn new() -> Policy {
    Policy {
      _embedded: None,
      _links: None,
      created: None,
      description: None,
      id: None,
      last_updated: None,
      name: None,
      priority: None,
      status: None,
      system: None,
      _type: None
    }
  }

  pub fn set_embedded(&mut self, _embedded: ::std::collections::HashMap<String, Value>) {
    self._embedded = Some(_embedded);
  }

  pub fn with_embedded(mut self, _embedded: ::std::collections::HashMap<String, Value>) -> Policy {
    self._embedded = Some(_embedded);
    self
  }

  pub fn _embedded(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self._embedded.as_ref()
  }

  pub fn reset_embedded(&mut self) {
    self._embedded = None;
  }

  pub fn set_links(&mut self, _links: ::std::collections::HashMap<String, Value>) {
    self._links = Some(_links);
  }

  pub fn with_links(mut self, _links: ::std::collections::HashMap<String, Value>) -> Policy {
    self._links = Some(_links);
    self
  }

  pub fn _links(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self._links.as_ref()
  }

  pub fn reset_links(&mut self) {
    self._links = None;
  }

  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> Policy {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> Policy {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> Policy {
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

  pub fn with_last_updated(mut self, last_updated: String) -> Policy {
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

  pub fn with_name(mut self, name: String) -> Policy {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_priority(&mut self, priority: i32) {
    self.priority = Some(priority);
  }

  pub fn with_priority(mut self, priority: i32) -> Policy {
    self.priority = Some(priority);
    self
  }

  pub fn priority(&self) -> Option<&i32> {
    self.priority.as_ref()
  }

  pub fn reset_priority(&mut self) {
    self.priority = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> Policy {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_system(&mut self, system: bool) {
    self.system = Some(system);
  }

  pub fn with_system(mut self, system: bool) -> Policy {
    self.system = Some(system);
    self
  }

  pub fn system(&self) -> Option<&bool> {
    self.system.as_ref()
  }

  pub fn reset_system(&mut self) {
    self.system = None;
  }

  pub fn set_type(&mut self, _type: crate::models::PolicyType) {
    self._type = Some(_type);
  }

  pub fn with_type(mut self, _type: crate::models::PolicyType) -> Policy {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&crate::models::PolicyType> {
    self._type.as_ref()
  }

  pub fn reset_type(&mut self) {
    self._type = None;
  }

}



