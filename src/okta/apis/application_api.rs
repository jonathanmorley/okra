use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct ApplicationApiClient {
    configuration: Configuration,
}

impl ApplicationApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_applications(
        &self,
        r#q: String,
        r#after: String,
        r#limit: i32,
        r#filter: String,
        r#expand: String,
        r#include_non_deleted: bool,
    ) -> Result<Vec<Application>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps".to_string(),
        )
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("filter".to_string(), r#filter.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .with_query_param("includeNonDeleted".to_string(), r#include_non_deleted.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_application(
        &self,
        r#activate: bool,
        r#body: Application,
    ) -> Result<Application, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps".to_string(),
        )
        .with_query_param("activate".to_string(), r#activate.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_application(
        &self,
        r#app_id: String,
        r#expand: String,
    ) -> Result<Application, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_application(
        &self,
        r#app_id: String,
        r#body: Application,
    ) -> Result<Application, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/apps/{appId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_application(
        &self,
        r#app_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/apps/{appId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_application_keys(
        &self,
        r#app_id: String,
    ) -> Result<Vec<JsonWebKey>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/credentials/keys".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_application_key(
        &self,
        r#app_id: String,
        r#key_id: String,
    ) -> Result<JsonWebKey, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/credentials/keys/{keyId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("keyId".to_string(), r#key_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#clone_application_key(
        &self,
        r#app_id: String,
        r#key_id: String,
        r#target_aid: String,
    ) -> Result<JsonWebKey, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/credentials/keys/{keyId}/clone".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("keyId".to_string(), r#key_id.to_string())
        .with_query_param("targetAid".to_string(), r#target_aid.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#list_application_group_assignments(
        &self,
        r#app_id: String,
        r#q: String,
        r#after: String,
        r#limit: i32,
        r#expand: String,
    ) -> Result<Vec<ApplicationGroupAssignment>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/groups".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_application_group_assignment(
        &self,
        r#app_id: String,
        r#group_id: String,
        r#expand: String,
    ) -> Result<ApplicationGroupAssignment, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_application_group_assignment(
        &self,
        r#app_id: String,
        r#group_id: String,
        r#body: ApplicationGroupAssignment,
    ) -> Result<ApplicationGroupAssignment, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_application_group_assignment(
        &self,
        r#app_id: String,
        r#group_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/apps/{appId}/groups/{groupId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_application(
        &self,
        r#app_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/lifecycle/activate".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_application(
        &self,
        r#app_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/lifecycle/deactivate".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_application_users(
        &self,
        r#app_id: String,
        r#q: String,
        r#query_scope: String,
        r#after: String,
        r#limit: i32,
        r#filter: String,
        r#expand: String,
    ) -> Result<Vec<AppUser>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/users".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("query_scope".to_string(), r#query_scope.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("filter".to_string(), r#filter.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#assign_user_to_application(
        &self,
        r#app_id: String,
        r#body: AppUser,
    ) -> Result<AppUser, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/users".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_application_user(
        &self,
        r#app_id: String,
        r#user_id: String,
        r#expand: String,
    ) -> Result<AppUser, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_application_user(
        &self,
        r#app_id: String,
        r#user_id: String,
        r#body: AppUser,
    ) -> Result<AppUser, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_application_user(
        &self,
        r#app_id: String,
        r#user_id: String,
        r#send_email: bool,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/apps/{appId}/users/{userId}".to_string(),
        )
        .with_path_param("appId".to_string(), r#app_id.to_string())
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
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
    fn r#list_applications() {
        client().r#list_applications(
          "q".into(),
          "after".into(),
          i32::default(),
          "filter".into(),
          "expand".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_application() {
        client().r#create_application(
          bool::default(),
          Application::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_application() {
        client().r#get_application(
          "appId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_application() {
        client().r#update_application(
          "appId".into(),
          Application::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_application() {
        client().r#delete_application(
          "appId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_application_keys() {
        client().r#list_application_keys(
          "appId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_application_key() {
        client().r#get_application_key(
          "appId".into(),
          "keyId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#clone_application_key() {
        client().r#clone_application_key(
          "appId".into(),
          "keyId".into(),
          "targetAid".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_application_group_assignments() {
        client().r#list_application_group_assignments(
          "appId".into(),
          "q".into(),
          "after".into(),
          i32::default(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_application_group_assignment() {
        client().r#get_application_group_assignment(
          "appId".into(),
          "groupId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_application_group_assignment() {
        client().r#create_application_group_assignment(
          "appId".into(),
          "groupId".into(),
          ApplicationGroupAssignment::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_application_group_assignment() {
        client().r#delete_application_group_assignment(
          "appId".into(),
          "groupId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_application() {
        client().r#activate_application(
          "appId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_application() {
        client().r#deactivate_application(
          "appId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_application_users() {
        client().r#list_application_users(
          "appId".into(),
          "q".into(),
          "query_scope".into(),
          "after".into(),
          i32::default(),
          "filter".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#assign_user_to_application() {
        client().r#assign_user_to_application(
          "appId".into(),
          AppUser::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_application_user() {
        client().r#get_application_user(
          "appId".into(),
          "userId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_application_user() {
        client().r#update_application_user(
          "appId".into(),
          "userId".into(),
          AppUser::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_application_user() {
        client().r#delete_application_user(
          "appId".into(),
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    

    fn client() -> super::ApplicationApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::ApplicationApiClient::new(configuration)
    }
}

