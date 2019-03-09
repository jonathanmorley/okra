#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#OktaSignOnPolicyRuleActions {
    #[serde(rename = "signon", skip_serializing_if = "Option::is_none")]
    r#signon: Option<OktaSignOnPolicyRuleSignonActions>,
}

impl r#OktaSignOnPolicyRuleActions {
    pub fn new(
    ) -> Self {
        Self {
          r#signon: None,
        }
    }

    pub fn set_signon(&mut self, r#signon: OktaSignOnPolicyRuleSignonActions) {
        self.r#signon = Some(r#signon);
    }

    pub fn with_signon(mut self, r#signon: OktaSignOnPolicyRuleSignonActions) -> Self {
        self.r#signon = Some(r#signon);
        self
    }

    pub fn r#signon(&self) -> Option<&OktaSignOnPolicyRuleSignonActions> {
        self.r#signon.as_ref().map(|x| x.borrow())
    }

    pub fn reset_signon(&mut self) {
        self.r#signon = None;
    }
}
