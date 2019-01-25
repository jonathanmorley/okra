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
pub struct EmailAddress {
  #[serde(rename = "status")]
  status: Option<crate::models::EmailStatus>,
  #[serde(rename = "type")]
  _type: Option<crate::models::EmailType>,
  #[serde(rename = "value")]
  value: Option<String>
}

impl EmailAddress {
  pub fn new() -> EmailAddress {
    EmailAddress {
      status: None,
      _type: None,
      value: None
    }
  }

  pub fn set_status(&mut self, status: crate::models::EmailStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: crate::models::EmailStatus) -> EmailAddress {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&crate::models::EmailStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_type(&mut self, _type: crate::models::EmailType) {
    self._type = Some(_type);
  }

  pub fn with_type(mut self, _type: crate::models::EmailType) -> EmailAddress {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&crate::models::EmailType> {
    self._type.as_ref()
  }

  pub fn reset_type(&mut self) {
    self._type = None;
  }

  pub fn set_value(&mut self, value: String) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: String) -> EmailAddress {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&String> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}



