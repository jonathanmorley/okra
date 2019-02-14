#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OAuthApplicationCredentials {
    #[serde(rename = "oauthClient", skip_serializing_if = "Option::is_none")]
    r#oauth_client: Option<ApplicationCredentialsOAuthClient>,
}

impl r#OAuthApplicationCredentials {
    pub fn new(
    ) -> Self {
        Self {
          r#oauth_client: None,
        }
    }

    pub fn set_oauth_client(&mut self, r#oauth_client: ApplicationCredentialsOAuthClient) {
        self.r#oauth_client = Some(r#oauth_client);
    }

    pub fn with_oauth_client(mut self, r#oauth_client: ApplicationCredentialsOAuthClient) -> Self {
        self.r#oauth_client = Some(r#oauth_client);
        self
    }

    pub fn r#oauth_client(&self) -> Option<&ApplicationCredentialsOAuthClient> {
        self.r#oauth_client.as_ref().map(|x| x.borrow())
    }

    pub fn reset_oauth_client(&mut self) {
        self.r#oauth_client = None;
    }
}
