#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogRequest {
    #[serde(rename = "ipChain", skip_serializing_if = "Option::is_none")]
    r#ip_chain: Option<Vec<LogIpAddress>>,
}

impl r#LogRequest {
    pub fn new(
    ) -> Self {
        Self {
          r#ip_chain: None,
        }
    }

    pub fn set_ip_chain(&mut self, r#ip_chain: Vec<LogIpAddress>) {
        self.r#ip_chain = Some(r#ip_chain);
    }

    pub fn with_ip_chain(mut self, r#ip_chain: Vec<LogIpAddress>) -> Self {
        self.r#ip_chain = Some(r#ip_chain);
        self
    }

    pub fn r#ip_chain(&self) -> Option<&Vec<LogIpAddress>> {
        self.r#ip_chain.as_ref().map(|x| x.borrow())
    }

    pub fn reset_ip_chain(&mut self) {
        self.r#ip_chain = None;
    }
}
