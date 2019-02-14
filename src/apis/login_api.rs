use std::borrow::Borrow;

use failure::Error;
use hyper;

use crate::okta::apis::configuration::Configuration;
use crate::okta::apis::request as _internal_request;

pub struct LoginApiClient {
    configuration: Configuration,
}

impl LoginApiClient {
    pub fn new(configuration: Configuration) -> LoginApiClient {
        LoginApiClient {
            configuration: configuration,
        }
    }
}

pub trait LoginApi {
    fn login_default(&self) -> Result<reqwest::Response, Error>;
    fn session_cookie_redirect(
        &self,
        check_account_setup_complete: bool,
        token: &str,
        redirect_url: &str,
    ) -> Result<reqwest::Response, Error>;
}

impl LoginApi for LoginApiClient {
    fn login_default(&self) -> Result<reqwest::Response, Error> {
        _internal_request::Request::new(hyper::Method::GET, "/login/default".to_string())
            .response(self.configuration.borrow())
    }

    fn session_cookie_redirect(
        &self,
        check_account_setup_complete: bool,
        token: &str,
        redirect_url: &str,
    ) -> Result<reqwest::Response, Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/login/sessionCookieRedirect".to_string(),
        )
        .with_query_param(
            "checkAccountSetupComplete".to_string(),
            check_account_setup_complete.to_string(),
        )
        .with_query_param("token".to_string(), token.to_string())
        .with_query_param("redirectUrl".to_string(), redirect_url.to_string())
        .response(self.configuration.borrow())
    }
}
