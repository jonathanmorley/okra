use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct SessionApiClient {
    configuration: configuration::Configuration,
}

impl SessionApiClient {
    pub fn new(configuration: configuration::Configuration) -> SessionApiClient {
        SessionApiClient {
            configuration: configuration,
        }
    }
}

pub trait SessionApi {
    fn create_session(
        &self,
        create_session_request: crate::models::CreateSessionRequest,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>>;
    fn end_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>>;
    fn refresh_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>>;
}

impl SessionApi for SessionApiClient {
    fn create_session(
        &self,
        create_session_request: crate::models::CreateSessionRequest,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/sessions".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_body_param(create_session_request)
            .execute(self.configuration.borrow())
    }

    fn end_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/sessions/{sessionId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("sessionId".to_string(), session_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn get_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/sessions/{sessionId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("sessionId".to_string(), session_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn refresh_session(
        &self,
        session_id: &str,
    ) -> Box<Future<Item = crate::models::Session, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/sessions/{sessionId}/lifecycle/refresh".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("sessionId".to_string(), session_id.to_string())
        .execute(self.configuration.borrow())
    }
}
