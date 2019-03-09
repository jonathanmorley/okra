use crate::okta::apis::configuration::Configuration;

pub struct APIClient {
    configuration: Configuration,
    application_api: Box<crate::okta::apis::ApplicationApiClient>,
    authentication_api: Box<crate::okta::apis::AuthenticationApiClient>,
    group_api: Box<crate::okta::apis::GroupApiClient>,
    log_api: Box<crate::okta::apis::LogApiClient>,
    login_api: Box<crate::apis::login_api::LoginApiClient>,
    policy_api: Box<crate::okta::apis::PolicyApiClient>,
    session_api: Box<crate::okta::apis::SessionApiClient>,
    user_api: Box<crate::okta::apis::UserApiClient>,
    user_factor_api: Box<crate::okta::apis::UserFactorApiClient>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        APIClient {
            configuration: configuration.clone(),
            application_api: Box::new(crate::okta::apis::ApplicationApiClient::new(
                configuration.clone(),
            )),
            authentication_api: Box::new(crate::okta::apis::AuthenticationApiClient::new(
                configuration.clone(),
            )),
            group_api: Box::new(crate::okta::apis::GroupApiClient::new(
                configuration.clone(),
            )),
            log_api: Box::new(crate::okta::apis::LogApiClient::new(configuration.clone())),
            login_api: Box::new(crate::apis::login_api::LoginApiClient::new(
                configuration.clone(),
            )),
            policy_api: Box::new(crate::okta::apis::PolicyApiClient::new(
                configuration.clone(),
            )),
            session_api: Box::new(crate::okta::apis::SessionApiClient::new(
                configuration.clone(),
            )),
            user_api: Box::new(crate::okta::apis::UserApiClient::new(configuration.clone())),
            user_factor_api: Box::new(crate::okta::apis::UserFactorApiClient::new(
                configuration.clone(),
            )),
        }
    }

    pub fn application_api(&self) -> &crate::okta::apis::ApplicationApiClient {
        self.application_api.as_ref()
    }

    pub fn authentication_api(&self) -> &crate::okta::apis::AuthenticationApiClient {
        self.authentication_api.as_ref()
    }

    pub fn group_api(&self) -> &crate::okta::apis::GroupApiClient {
        self.group_api.as_ref()
    }

    pub fn log_api(&self) -> &crate::okta::apis::LogApiClient {
        self.log_api.as_ref()
    }

    pub fn login_api(&self) -> &crate::apis::login_api::LoginApiClient {
        self.login_api.as_ref()
    }

    pub fn policy_api(&self) -> &crate::okta::apis::PolicyApiClient {
        self.policy_api.as_ref()
    }

    pub fn session_api(&self) -> &crate::okta::apis::SessionApiClient {
        self.session_api.as_ref()
    }

    pub fn user_api(&self) -> &crate::okta::apis::UserApiClient {
        self.user_api.as_ref()
    }

    pub fn user_factor_api(&self) -> &crate::okta::apis::UserFactorApiClient {
        self.user_factor_api.as_ref()
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
