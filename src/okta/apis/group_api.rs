use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct GroupApiClient {
    configuration: Configuration,
}

impl GroupApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_groups(
        &self,
        r#q: String,
        r#filter: String,
        r#after: String,
        r#limit: i32,
        r#expand: String,
    ) -> Result<Vec<Group>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups".to_string(),
        )
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("filter".to_string(), r#filter.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_group(
        &self,
        r#body: Group,
    ) -> Result<Group, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#list_rules(
        &self,
        r#limit: i32,
        r#after: String,
        r#expand: String,
    ) -> Result<Vec<GroupRule>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/rules".to_string(),
        )
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_rule(
        &self,
        r#body: GroupRule,
    ) -> Result<GroupRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups/rules".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_rule(
        &self,
        r#rule_id: String,
        r#expand: String,
    ) -> Result<GroupRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_rule(
        &self,
        r#rule_id: String,
        r#body: GroupRule,
    ) -> Result<GroupRule, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_rule(
        &self,
        r#rule_id: String,
        r#remove_users: bool,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/rules/{ruleId}".to_string(),
        )
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .with_query_param("removeUsers".to_string(), r#remove_users.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_rule(
        &self,
        r#rule_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups/rules/{ruleId}/lifecycle/activate".to_string(),
        )
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_rule(
        &self,
        r#rule_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/groups/rules/{ruleId}/lifecycle/deactivate".to_string(),
        )
        .with_path_param("ruleId".to_string(), r#rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#get_group(
        &self,
        r#group_id: String,
        r#expand: String,
    ) -> Result<Group, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/{groupId}".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_group(
        &self,
        r#group_id: String,
        r#body: Group,
    ) -> Result<Group, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/groups/{groupId}".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_group(
        &self,
        r#group_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/{groupId}".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_group_users(
        &self,
        r#group_id: String,
        r#after: String,
        r#limit: i32,
        r#managed_by: String,
    ) -> Result<Vec<User>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/groups/{groupId}/users".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("managedBy".to_string(), r#managed_by.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#add_user_to_group(
        &self,
        r#group_id: String,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/groups/{groupId}/users/{userId}".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#remove_group_user(
        &self,
        r#group_id: String,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/groups/{groupId}/users/{userId}".to_string(),
        )
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_path_param("userId".to_string(), r#user_id.to_string())
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
    fn r#list_groups() {
        client().r#list_groups(
          "q".into(),
          "filter".into(),
          "after".into(),
          i32::default(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_group() {
        client().r#create_group(
          Group::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_rules() {
        client().r#list_rules(
          i32::default(),
          "after".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_rule() {
        client().r#create_rule(
          GroupRule::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_rule() {
        client().r#get_rule(
          "ruleId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_rule() {
        client().r#update_rule(
          "ruleId".into(),
          GroupRule::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_rule() {
        client().r#delete_rule(
          "ruleId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_rule() {
        client().r#activate_rule(
          "ruleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_rule() {
        client().r#deactivate_rule(
          "ruleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_group() {
        client().r#get_group(
          "groupId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_group() {
        client().r#update_group(
          "groupId".into(),
          Group::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_group() {
        client().r#delete_group(
          "groupId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_group_users() {
        client().r#list_group_users(
          "groupId".into(),
          "after".into(),
          i32::default(),
          "managedBy".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#add_user_to_group() {
        client().r#add_user_to_group(
          "groupId".into(),
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#remove_group_user() {
        client().r#remove_group_user(
          "groupId".into(),
          "userId".into(),
        ).unwrap();
    }

    

    fn client() -> super::GroupApiClient {
        std::process::Command::new("docker")
                  .args(&["build", "-t=test-apisprout", "."])
                  .output()
                  .expect("failed to execute process");

        let testcontainer_docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = testcontainer_docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::GroupApiClient::new(configuration)
    }
}

