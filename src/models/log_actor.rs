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
pub struct LogActor {
  #[serde(rename = "alternateId")]
  alternate_id: Option<String>,
  #[serde(rename = "detail")]
  detail: Option<::std::collections::HashMap<String, Value>>,
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>
}

impl LogActor {
  pub fn new() -> LogActor {
    LogActor {
      alternate_id: None,
      detail: None,
      display_name: None,
      id: None,
      _type: None
    }
  }

  pub fn set_alternate_id(&mut self, alternate_id: String) {
    self.alternate_id = Some(alternate_id);
  }

  pub fn with_alternate_id(mut self, alternate_id: String) -> LogActor {
    self.alternate_id = Some(alternate_id);
    self
  }

  pub fn alternate_id(&self) -> Option<&String> {
    self.alternate_id.as_ref()
  }

  pub fn reset_alternate_id(&mut self) {
    self.alternate_id = None;
  }

  pub fn set_detail(&mut self, detail: ::std::collections::HashMap<String, Value>) {
    self.detail = Some(detail);
  }

  pub fn with_detail(mut self, detail: ::std::collections::HashMap<String, Value>) -> LogActor {
    self.detail = Some(detail);
    self
  }

  pub fn detail(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.detail.as_ref()
  }

  pub fn reset_detail(&mut self) {
    self.detail = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> LogActor {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> LogActor {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with_type(mut self, _type: String) -> LogActor {
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



