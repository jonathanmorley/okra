use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct SessionApiClient {
    configuration: Configuration,
}

impl SessionApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#create_session(
        &self,
        r#body: CreateSessionRequest,
    ) -> Result<Session, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/sessions".to_string(),
        )
        .with_body_param(r#body)
        .execute(self.configuration.borrow())
    }

    pub fn r#get_session(
        &self,
        r#session_id: String,
    ) -> Result<Session, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/sessions/{sessionId}".to_string(),
        )
        .with_path_param("sessionId".to_string(), r#session_id.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#end_session(
        &self,
        r#session_id: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/sessions/{sessionId}".to_string(),
        )
        .with_path_param("sessionId".to_string(), r#session_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#refresh_session(
        &self,
        r#session_id: String,
    ) -> Result<Session, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/sessions/{sessionId}/lifecycle/refresh".to_string(),
        )
        .with_path_param("sessionId".to_string(), r#session_id.to_string())
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
    fn r#create_session() {
        client().r#create_session(
          CreateSessionRequest::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_session() {
        client().r#get_session(
          "sessionId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#end_session() {
        client().r#end_session(
          "sessionId".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#refresh_session() {
        client().r#refresh_session(
          "sessionId".into(),
        ).unwrap();
    }

    

    fn client() -> super::SessionApiClient {
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
        super::SessionApiClient::new(configuration)
    }
}

