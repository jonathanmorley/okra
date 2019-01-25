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
pub struct SessionIdentityProvider {
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "type")]
  _type: Option<crate::models::SessionIdentityProviderType>
}

impl SessionIdentityProvider {
  pub fn new() -> SessionIdentityProvider {
    SessionIdentityProvider {
      id: None,
      _type: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SessionIdentityProvider {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_type(&mut self, _type: crate::models::SessionIdentityProviderType) {
    self._type = Some(_type);
  }

  pub fn with_type(mut self, _type: crate::models::SessionIdentityProviderType) -> SessionIdentityProvider {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&crate::models::SessionIdentityProviderType> {
    self._type.as_ref()
  }

  pub fn reset_type(&mut self) {
    self._type = None;
  }

}



