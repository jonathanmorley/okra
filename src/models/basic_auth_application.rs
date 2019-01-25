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
pub struct BasicAuthApplication {
  #[serde(rename = "credentials")]
  credentials: Option<crate::models::SchemeApplicationCredentials>,
  #[serde(rename = "name")]
  name: Option<Value>,
  #[serde(rename = "settings")]
  settings: Option<crate::models::BasicApplicationSettings>
}

impl BasicAuthApplication {
  pub fn new() -> BasicAuthApplication {
    BasicAuthApplication {
      credentials: None,
      name: None,
      settings: None
    }
  }

  pub fn set_credentials(&mut self, credentials: crate::models::SchemeApplicationCredentials) {
    self.credentials = Some(credentials);
  }

  pub fn with_credentials(mut self, credentials: crate::models::SchemeApplicationCredentials) -> BasicAuthApplication {
    self.credentials = Some(credentials);
    self
  }

  pub fn credentials(&self) -> Option<&crate::models::SchemeApplicationCredentials> {
    self.credentials.as_ref()
  }

  pub fn reset_credentials(&mut self) {
    self.credentials = None;
  }

  pub fn set_name(&mut self, name: Value) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: Value) -> BasicAuthApplication {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&Value> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_settings(&mut self, settings: crate::models::BasicApplicationSettings) {
    self.settings = Some(settings);
  }

  pub fn with_settings(mut self, settings: crate::models::BasicApplicationSettings) -> BasicAuthApplication {
    self.settings = Some(settings);
    self
  }

  pub fn settings(&self) -> Option<&crate::models::BasicApplicationSettings> {
    self.settings.as_ref()
  }

  pub fn reset_settings(&mut self) {
    self.settings = None;
  }

}


