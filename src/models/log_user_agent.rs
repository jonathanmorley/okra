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
pub struct LogUserAgent {
  #[serde(rename = "browser")]
  browser: Option<String>,
  #[serde(rename = "os")]
  os: Option<String>,
  #[serde(rename = "rawUserAgent")]
  raw_user_agent: Option<String>
}

impl LogUserAgent {
  pub fn new() -> LogUserAgent {
    LogUserAgent {
      browser: None,
      os: None,
      raw_user_agent: None
    }
  }

  pub fn set_browser(&mut self, browser: String) {
    self.browser = Some(browser);
  }

  pub fn with_browser(mut self, browser: String) -> LogUserAgent {
    self.browser = Some(browser);
    self
  }

  pub fn browser(&self) -> Option<&String> {
    self.browser.as_ref()
  }

  pub fn reset_browser(&mut self) {
    self.browser = None;
  }

  pub fn set_os(&mut self, os: String) {
    self.os = Some(os);
  }

  pub fn with_os(mut self, os: String) -> LogUserAgent {
    self.os = Some(os);
    self
  }

  pub fn os(&self) -> Option<&String> {
    self.os.as_ref()
  }

  pub fn reset_os(&mut self) {
    self.os = None;
  }

  pub fn set_raw_user_agent(&mut self, raw_user_agent: String) {
    self.raw_user_agent = Some(raw_user_agent);
  }

  pub fn with_raw_user_agent(mut self, raw_user_agent: String) -> LogUserAgent {
    self.raw_user_agent = Some(raw_user_agent);
    self
  }

  pub fn raw_user_agent(&self) -> Option<&String> {
    self.raw_user_agent.as_ref()
  }

  pub fn reset_raw_user_agent(&mut self) {
    self.raw_user_agent = None;
  }

}


