use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct UserFactorApiClient {
    configuration: configuration::Configuration,
}

impl UserFactorApiClient {
    pub fn new(configuration: configuration::Configuration) -> UserFactorApiClient {
        UserFactorApiClient {
            configuration: configuration,
        }
    }
}

pub trait UserFactorApi {
    fn activate_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        body: crate::models::VerifyFactorRequest,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>>;
    fn add_factor(
        &self,
        user_id: &str,
        body: crate::models::Factor,
        update_phone: bool,
        template_id: &str,
        token_lifetime_seconds: i32,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>>;
    fn delete_factor(
        &self,
        user_id: &str,
        factor_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_factor(
        &self,
        user_id: &str,
        factor_id: &str,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>>;
    fn list_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::Factor>, Error = Error<serde_json::Value>>>;
    fn list_supported_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::Factor>, Error = Error<serde_json::Value>>>;
    fn list_supported_security_questions(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::SecurityQuestion>, Error = Error<serde_json::Value>>>;
    fn verify_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        body: crate::models::VerifyFactorRequest,
        template_id: &str,
        token_lifetime_seconds: i32,
    ) -> Box<Future<Item = crate::models::VerifyFactorResponse, Error = Error<serde_json::Value>>>;
}

impl UserFactorApi for UserFactorApiClient {
    fn activate_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        body: crate::models::VerifyFactorRequest,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors/{factorId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }

    fn add_factor(
        &self,
        user_id: &str,
        body: crate::models::Factor,
        update_phone: bool,
        template_id: &str,
        token_lifetime_seconds: i32,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("updatePhone".to_string(), update_phone.to_string())
        .with_query_param("templateId".to_string(), template_id.to_string())
        .with_query_param(
            "tokenLifetimeSeconds".to_string(),
            token_lifetime_seconds.to_string(),
        )
        .with_query_param("activate".to_string(), activate.to_string())
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }

    fn delete_factor(
        &self,
        user_id: &str,
        factor_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/users/{userId}/factors/{factorId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn get_factor(
        &self,
        user_id: &str,
        factor_id: &str,
    ) -> Box<Future<Item = crate::models::Factor, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/{factorId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::Factor>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_supported_factors(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::Factor>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/catalog".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_supported_security_questions(
        &self,
        user_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::SecurityQuestion>, Error = Error<serde_json::Value>>>
    {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/users/{userId}/factors/questions".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("userId".to_string(), user_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn verify_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        body: crate::models::VerifyFactorRequest,
        template_id: &str,
        token_lifetime_seconds: i32,
    ) -> Box<Future<Item = crate::models::VerifyFactorResponse, Error = Error<serde_json::Value>>>
    {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/users/{userId}/factors/{factorId}/verify".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("templateId".to_string(), template_id.to_string())
        .with_query_param(
            "tokenLifetimeSeconds".to_string(),
            token_lifetime_seconds.to_string(),
        )
        .with_path_param("userId".to_string(), user_id.to_string())
        .with_path_param("factorId".to_string(), factor_id.to_string())
        .with_body_param(body)
        .execute(self.configuration.borrow())
    }
}
