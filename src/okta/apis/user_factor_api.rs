use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct UserFactorApiClient {
    configuration: Configuration,
}

impl UserFactorApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_factors(
        &self,
        r#user_id: String,
    ) -> Result<Vec<Factor>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#add_factor(
        &self,
        r#user_id: String,
        r#update_phone: bool,
        r#template_id: String,
        r#token_lifetime_seconds: i32,
        r#activate: bool,
        r#body: Factor,
    ) -> Result<Factor, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_query_param("updatePhone".to_string(), r#update_phone.to_string())
        .with_query_param("templateId".to_string(), r#template_id.to_string())
        .with_query_param("tokenLifetimeSeconds".to_string(), r#token_lifetime_seconds.to_string())
        .with_query_param("activate".to_string(), r#activate.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#list_supported_factors(
        &self,
        r#user_id: String,
    ) -> Result<Vec<Factor>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/catalog".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#list_supported_security_questions(
        &self,
        r#user_id: String,
    ) -> Result<Vec<SecurityQuestion>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/questions".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_factor(
        &self,
        r#user_id: String,
        r#factor_id: String,
    ) -> Result<Factor, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/{factorId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#delete_factor(
        &self,
        r#user_id: String,
        r#factor_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/factors/{factorId}".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#activate_factor(
        &self,
        r#user_id: String,
        r#factor_id: String,
        r#body: VerifyFactorRequest,
    ) -> Result<Factor, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors/{factorId}/lifecycle/activate".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#verify_factor(
        &self,
        r#user_id: String,
        r#factor_id: String,
        r#template_id: String,
        r#token_lifetime_seconds: i32,
        r#body: VerifyFactorRequest,
    ) -> Result<VerifyFactorResponse, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors/{factorId}/verify".to_string(),
        )
        .with_path_param("userId".to_string(), r#user_id.to_string())
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .with_query_param("templateId".to_string(), r#template_id.to_string())
        .with_query_param("tokenLifetimeSeconds".to_string(), r#token_lifetime_seconds.to_string())
        .with_body_param(r#body)
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
    fn r#list_factors() {
        client().r#list_factors(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#add_factor() {
        client().r#add_factor(
          "userId".into(),
          bool::default(),
          "templateId".into(),
          i32::default(),
          bool::default(),
          Factor::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_supported_factors() {
        client().r#list_supported_factors(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#list_supported_security_questions() {
        client().r#list_supported_security_questions(
          "userId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_factor() {
        client().r#get_factor(
          "userId".into(),
          "factorId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#delete_factor() {
        client().r#delete_factor(
          "userId".into(),
          "factorId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#activate_factor() {
        client().r#activate_factor(
          "userId".into(),
          "factorId".into(),
          VerifyFactorRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#verify_factor() {
        client().r#verify_factor(
          "userId".into(),
          "factorId".into(),
          "templateId".into(),
          i32::default(),
          VerifyFactorRequest::default(),
        ).unwrap();
    }

    

    fn client() -> super::UserFactorApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::UserFactorApiClient::new(configuration)
    }
}

