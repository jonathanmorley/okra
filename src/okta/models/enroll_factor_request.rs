#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#EnrollFactorRequest {
    #[serde(rename = "factorType")]
    r#factor_type: FactorType,
    #[serde(rename = "profile")]
    r#profile: FactorProfile,
    #[serde(rename = "provider")]
    r#provider: FactorProvider,
    #[serde(rename = "stateToken")]
    r#state_token: String,
}

impl r#EnrollFactorRequest {
    pub fn new(
        r#factor_type: FactorType,
        r#profile: FactorProfile,
        r#provider: FactorProvider,
        r#state_token: String,
    ) -> Self {
        Self {
          r#factor_type: factor_type,
          r#profile: profile,
          r#provider: provider,
          r#state_token: state_token,
        }
    }

    pub fn set_factor_type(&mut self, r#factor_type: FactorType) {
        self.r#factor_type = r#factor_type;
    }

    pub fn with_factor_type(mut self, r#factor_type: FactorType) -> Self {
        self.r#factor_type = r#factor_type;
        self
    }

    pub fn r#factor_type(&self) -> &FactorType {
        self.r#factor_type.borrow()
    }

    pub fn set_profile(&mut self, r#profile: FactorProfile) {
        self.r#profile = r#profile;
    }

    pub fn with_profile(mut self, r#profile: FactorProfile) -> Self {
        self.r#profile = r#profile;
        self
    }

    pub fn r#profile(&self) -> &FactorProfile {
        self.r#profile.borrow()
    }

    pub fn set_provider(&mut self, r#provider: FactorProvider) {
        self.r#provider = r#provider;
    }

    pub fn with_provider(mut self, r#provider: FactorProvider) -> Self {
        self.r#provider = r#provider;
        self
    }

    pub fn r#provider(&self) -> &FactorProvider {
        self.r#provider.borrow()
    }

    pub fn set_state_token(&mut self, r#state_token: String) {
        self.r#state_token = r#state_token;
    }

    pub fn with_state_token(mut self, r#state_token: String) -> Self {
        self.r#state_token = r#state_token;
        self
    }

    pub fn r#state_token(&self) -> &str {
        self.r#state_token.borrow()
    }
}
