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
pub struct CreateSessionRequest {
  #[serde(rename = "sessionToken")]
  session_token: Option<String>
}

impl CreateSessionRequest {
  pub fn new() -> CreateSessionRequest {
    CreateSessionRequest {
      session_token: None
    }
  }

  pub fn set_session_token(&mut self, session_token: String) {
    self.session_token = Some(session_token);
  }

  pub fn with_session_token(mut self, session_token: String) -> CreateSessionRequest {
    self.session_token = Some(session_token);
    self
  }

  pub fn session_token(&self) -> Option<&String> {
    self.session_token.as_ref()
  }

  pub fn reset_session_token(&mut self) {
    self.session_token = None;
  }

}


