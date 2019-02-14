use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct AuthenticationApiClient {
    configuration: Configuration,
}

impl AuthenticationApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#authenticate(
        &self,
        r#body: AuthenticationRequest,
    ) -> Result<AuthenticationTransaction, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#auth_change_password(
        &self,
        r#body: ChangePasswordRequest,
    ) -> Result<AuthenticationTransaction, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/credentials/change_password".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#enroll_factor(
        &self,
        r#body: EnrollFactorRequest,
    ) -> Result<AuthenticationTransaction, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/factors".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#auth_activate_factor(
        &self,
        r#factor_id: String,
        r#body: ActivateFactorRequest,
    ) -> Result<AuthenticationTransaction, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/factors/{factorId}/lifecycle/activate".to_string(),
        )
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#auth_verify_factor(
        &self,
        r#factor_id: String,
        r#remember_device: bool,
        r#auto_push: bool,
        r#body: AuthVerifyFactorRequest,
    ) -> Result<AuthenticationTransaction, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/authn/factors/{factorId}/verify".to_string(),
        )
        .with_path_param("factorId".to_string(), r#factor_id.to_string())
        .with_query_param("rememberDevice".to_string(), r#remember_device.to_string())
        .with_query_param("autoPush".to_string(), r#auto_push.to_string())
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
    fn r#authenticate() {
        client().r#authenticate(
          AuthenticationRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#auth_change_password() {
        client().r#auth_change_password(
          ChangePasswordRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#enroll_factor() {
        client().r#enroll_factor(
          EnrollFactorRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#auth_activate_factor() {
        client().r#auth_activate_factor(
          "factorId".into(),
          ActivateFactorRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#auth_verify_factor() {
        client().r#auth_verify_factor(
          "factorId".into(),
          bool::default(),
          bool::default(),
          AuthVerifyFactorRequest::default(),
        ).unwrap();
    }

    

    fn client() -> super::AuthenticationApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::AuthenticationApiClient::new(configuration)
    }
}

