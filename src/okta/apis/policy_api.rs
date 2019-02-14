use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct PolicyApiClient {
    configuration: Configuration,
}

impl PolicyApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_policies(
        &self,
        r#type: String,
        r#status: String,
        r#after: String,
        r#limit: i32,
        r#expand: String,
    ) -> Result<Vec<Policy>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies".to_string(),
        )
        .with_query_param("type".to_string(), r#type.to_string())
        .with_query_param("status".to_string(), r#status.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_policy(
        &self,
        r#activate: bool,
        r#body: Policy,
    ) -> Result<Policy, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies".to_string(),
        )
        .with_query_param("activate".to_string(), r#activate.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_policy(
        &self,
        r#policy_id: String,
        r#expand: String,
    ) -> Result<Policy, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_policy(
        &self,
        r#policy_id: String,
        r#body: Policy,
    ) -> Result<Policy, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_policy(
        &self,
        r#policy_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_policy(
        &self,
        r#policy_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/lifecycle/activate".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_policy(
        &self,
        r#policy_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/lifecycle/deactivate".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_policy_rules(
        &self,
        r#policy_id: String,
    ) -> Result<Vec<PolicyRule>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}/rules".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#add_policy_rule(
        &self,
        r#policy_id: String,
        r#activate: bool,
        r#body: PolicyRule,
    ) -> Result<PolicyRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_query_param("activate".to_string(), r#activate.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_policy_rule(
        &self,
        r#policy_id: String,
        r#rule_id: String,
    ) -> Result<PolicyRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_policy_rule(
        &self,
        r#policy_id: String,
        r#rule_id: String,
        r#body: PolicyRule,
    ) -> Result<PolicyRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_policy_rule(
        &self,
        r#policy_id: String,
        r#rule_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_policy_rule(
        &self,
        r#policy_id: String,
        r#rule_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/activate".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_policy_rule(
        &self,
        r#policy_id: String,
        r#rule_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/deactivate".to_string(),
        )
        .with_path_param("policyId".to_string(), r#policy_id.to_string())
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use tc_core::{Container, Image};
    use tc_generic::{GenericImage, WaitFor};
    use testcontainers::*;
    #[test]
    fn r#list_policies() {
        client().r#list_policies(
          "type".into(),
          "status".into(),
          "after".into(),
          i32::default(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_policy() {
        client().r#create_policy(
          bool::default(),
          Policy::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_policy() {
        client().r#get_policy(
          "policyId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_policy() {
        client().r#update_policy(
          "policyId".into(),
          Policy::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_policy() {
        client().r#delete_policy(
          "policyId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_policy() {
        client().r#activate_policy(
          "policyId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_policy() {
        client().r#deactivate_policy(
          "policyId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_policy_rules() {
        client().r#list_policy_rules(
          "policyId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#add_policy_rule() {
        client().r#add_policy_rule(
          "policyId".into(),
          bool::default(),
          PolicyRule::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_policy_rule() {
        client().r#get_policy_rule(
          "policyId".into(),
          "ruleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_policy_rule() {
        client().r#update_policy_rule(
          "policyId".into(),
          "ruleId".into(),
          PolicyRule::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_policy_rule() {
        client().r#delete_policy_rule(
          "policyId".into(),
          "ruleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_policy_rule() {
        client().r#activate_policy_rule(
          "policyId".into(),
          "ruleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_policy_rule() {
        client().r#deactivate_policy_rule(
          "policyId".into(),
          "ruleId".into(),
        ).unwrap();
    }

    

    fn client() -> super::PolicyApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::PolicyApiClient::new(configuration)
    }
}

