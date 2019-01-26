use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    application_api: Box<crate::apis::ApplicationApi>,
    authentication_api: Box<crate::apis::AuthenticationApi>,
    group_api: Box<crate::apis::GroupApi>,
    log_api: Box<crate::apis::LogApi>,
    policy_api: Box<crate::apis::PolicyApi>,
    session_api: Box<crate::apis::SessionApi>,
    user_api: Box<crate::apis::UserApi>,
    user_factor_api: Box<crate::apis::UserFactorApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        let rc = Rc::new(configuration);

        APIClient {
            application_api: Box::new(crate::apis::ApplicationApiClient::new(rc.clone())),
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
            group_api: Box::new(crate::apis::GroupApiClient::new(rc.clone())),
            log_api: Box::new(crate::apis::LogApiClient::new(rc.clone())),
            policy_api: Box::new(crate::apis::PolicyApiClient::new(rc.clone())),
            session_api: Box::new(crate::apis::SessionApiClient::new(rc.clone())),
            user_api: Box::new(crate::apis::UserApiClient::new(rc.clone())),
            user_factor_api: Box::new(crate::apis::UserFactorApiClient::new(rc.clone())),
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
}
