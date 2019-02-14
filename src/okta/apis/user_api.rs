use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct UserApiClient {
    configuration: Configuration,
}

impl UserApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_users(
        &self,
        r#q: String,
        r#after: String,
        r#limit: i32,
        r#filter: String,
        r#format: String,
        r#search: String,
        r#expand: String,
    ) -> Result<Vec<User>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users".to_string(),
        )
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("filter".to_string(), r#filter.to_string())
        .with_query_param("format".to_string(), r#format.to_string())
        .with_query_param("search".to_string(), r#search.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_user(
        &self,
        r#activate: bool,
        r#provider: bool,
        r#next_login: String,
        r#body: User,
    ) -> Result<User, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users".to_string(),
        )
        .with_query_param("activate".to_string(), r#activate.to_string())
        .with_query_param("provider".to_string(), r#provider.to_string())
        .with_query_param("nextLogin".to_string(), r#next_login.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_user(
        &self,
        r#user_id: String,
    ) -> Result<User, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#update_user(
        &self,
        r#user_id: String,
        r#body: User,
    ) -> Result<User, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/users/{userId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_or_delete_user(
        &self,
        r#user_id: String,
        r#send_email: bool,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_app_links(
        &self,
        r#user_id: String,
        r#show_all: bool,
    ) -> Result<Vec<AppLink>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/appLinks".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("showAll".to_string(), r#show_all.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#change_password(
        &self,
        r#user_id: String,
        r#body: ChangePasswordRequest,
    ) -> Result<UserCredentials, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/change_password".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#change_recovery_question(
        &self,
        r#user_id: String,
        r#body: UserCredentials,
    ) -> Result<UserCredentials, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/change_recovery_question".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#forgot_password(
        &self,
        r#user_id: String,
        r#send_email: bool,
        r#body: UserCredentials,
    ) -> Result<ForgotPasswordResponse, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/credentials/forgot_password".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#list_user_groups(
        &self,
        r#user_id: String,
        r#after: String,
        r#limit: i32,
    ) -> Result<Vec<Group>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/groups".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_user(
        &self,
        r#user_id: String,
        r#send_email: bool,
    ) -> Result<UserActivationToken, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/activate".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#deactivate_user(
        &self,
        r#user_id: String,
        r#send_email: bool,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/deactivate".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#expire_password(
        &self,
        r#user_id: String,
        r#temp_password: bool,
    ) -> Result<TempPassword, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/expire_password".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("tempPassword".to_string(), r#temp_password.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#reset_all_factors(
        &self,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/reset_factors".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#reset_password(
        &self,
        r#user_id: String,
        r#provider: String,
        r#send_email: bool,
    ) -> Result<ResetPasswordToken, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/reset_password".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("provider".to_string(), r#provider.to_string())
        .with_query_param("sendEmail".to_string(), r#send_email.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#suspend_user(
        &self,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/suspend".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#unlock_user(
        &self,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/unlock".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#unsuspend_user(
        &self,
        r#user_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/lifecycle/unsuspend".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_assigned_roles(
        &self,
        r#user_id: String,
        r#expand: String,
    ) -> Result<Vec<Role>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/roles".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("expand".to_string(), r#expand.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#add_role_to_user(
        &self,
        r#user_id: String,
        r#body: Role,
    ) -> Result<Role, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/roles".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#remove_role_from_user(
        &self,
        r#user_id: String,
        r#role_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/roles/{roleId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("roleId".to_string(), r#role_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#list_group_targets_for_role(
        &self,
        r#user_id: String,
        r#role_id: String,
        r#after: String,
        r#limit: i32,
    ) -> Result<Vec<Group>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("roleId".to_string(), r#role_id.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#add_group_target_to_role(
        &self,
        r#user_id: String,
        r#role_id: String,
        r#group_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("roleId".to_string(), r#role_id.to_string())
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#remove_group_target_from_role(
        &self,
        r#user_id: String,
        r#role_id: String,
        r#group_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("roleId".to_string(), r#role_id.to_string())
        .with_path_param("groupId".to_string(), r#group_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#end_all_user_sessions(
        &self,
        r#user_id: String,
        r#oauth_tokens: bool,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/sessions".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("oauthTokens".to_string(), r#oauth_tokens.to_string())
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
    fn r#list_users() {
        client().r#list_users(
          "q".into(),
          "after".into(),
          i32::default(),
          "filter".into(),
          "format".into(),
          "search".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_user() {
        client().r#create_user(
          bool::default(),
          bool::default(),
          "nextLogin".into(),
          User::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_user() {
        client().r#get_user(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#update_user() {
        client().r#update_user(
          "userId".into(),
          User::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_or_delete_user() {
        client().r#deactivate_or_delete_user(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_app_links() {
        client().r#list_app_links(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#change_password() {
        client().r#change_password(
          "userId".into(),
          ChangePasswordRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#change_recovery_question() {
        client().r#change_recovery_question(
          "userId".into(),
          UserCredentials::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#forgot_password() {
        client().r#forgot_password(
          "userId".into(),
          bool::default(),
          UserCredentials::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_user_groups() {
        client().r#list_user_groups(
          "userId".into(),
          "after".into(),
          i32::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_user() {
        client().r#activate_user(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#deactivate_user() {
        client().r#deactivate_user(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#expire_password() {
        client().r#expire_password(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#reset_all_factors() {
        client().r#reset_all_factors(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#reset_password() {
        client().r#reset_password(
          "userId".into(),
          "provider".into(),
          bool::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#suspend_user() {
        client().r#suspend_user(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#unlock_user() {
        client().r#unlock_user(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#unsuspend_user() {
        client().r#unsuspend_user(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_assigned_roles() {
        client().r#list_assigned_roles(
          "userId".into(),
          "expand".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#add_role_to_user() {
        client().r#add_role_to_user(
          "userId".into(),
          Role::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#remove_role_from_user() {
        client().r#remove_role_from_user(
          "userId".into(),
          "roleId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_group_targets_for_role() {
        client().r#list_group_targets_for_role(
          "userId".into(),
          "roleId".into(),
          "after".into(),
          i32::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#add_group_target_to_role() {
        client().r#add_group_target_to_role(
          "userId".into(),
          "roleId".into(),
          "groupId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#remove_group_target_from_role() {
        client().r#remove_group_target_from_role(
          "userId".into(),
          "roleId".into(),
          "groupId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#end_all_user_sessions() {
        client().r#end_all_user_sessions(
          "userId".into(),
          bool::default(),
        ).unwrap();
    }

    

    fn client() -> super::UserApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::UserApiClient::new(configuration)
    }
}

