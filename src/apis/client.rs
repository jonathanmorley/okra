use crate::apis::configuration::Configuration;

pub struct APIClient {
    configuration: Configuration,
    application_api: Box<crate::apis::ApplicationApi>,
    authentication_api: Box<crate::apis::AuthenticationApi>,
    group_api: Box<crate::apis::GroupApi>,
    log_api: Box<crate::apis::LogApi>,
    login_api: Box<crate::apis::LoginApi>,
    policy_api: Box<crate::apis::PolicyApi>,
    session_api: Box<crate::apis::SessionApi>,
    user_api: Box<crate::apis::UserApi>,
    user_factor_api: Box<crate::apis::UserFactorApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        APIClient {
            configuration: configuration.clone(),
            application_api: Box::new(crate::apis::ApplicationApiClient::new(
                configuration.clone(),
            )),
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(
                configuration.clone(),
            )),
            group_api: Box::new(crate::apis::GroupApiClient::new(configuration.clone())),
            log_api: Box::new(crate::apis::LogApiClient::new(configuration.clone())),
            login_api: Box::new(crate::apis::LoginApiClient::new(configuration.clone())),
            policy_api: Box::new(crate::apis::PolicyApiClient::new(configuration.clone())),
            session_api: Box::new(crate::apis::SessionApiClient::new(configuration.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(configuration.clone())),
            user_factor_api: Box::new(crate::apis::UserFactorApiClient::new(configuration.clone())),
        }
    }

    pub fn application_api(&self) -> &crate::apis::ApplicationApi {
        self.application_api.as_ref()
    }

    pub fn authentication_api(&self) -> &crate::apis::AuthenticationApi {
        self.authentication_api.as_ref()
    }

    pub fn group_api(&self) -> &crate::apis::GroupApi {
        self.group_api.as_ref()
    }

    pub fn log_api(&self) -> &crate::apis::LogApi {
        self.log_api.as_ref()
    }

    pub fn login_api(&self) -> &crate::apis::LoginApi {
        self.login_api.as_ref()
    }

    pub fn policy_api(&self) -> &crate::apis::PolicyApi {
        self.policy_api.as_ref()
    }

    pub fn session_api(&self) -> &crate::apis::SessionApi {
        self.session_api.as_ref()
    }

    pub fn user_api(&self) -> &crate::apis::UserApi {
        self.user_api.as_ref()
    }

    pub fn user_factor_api(&self) -> &crate::apis::UserFactorApi {
        self.user_factor_api.as_ref()
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
