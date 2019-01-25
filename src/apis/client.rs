use std::rc::Rc;

use super::configuration::Configuration;
use hyper;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    application_api: Box<crate::apis::ApplicationApi>,
    group_api: Box<crate::apis::GroupApi>,
    log_api: Box<crate::apis::LogApi>,
    policy_api: Box<crate::apis::PolicyApi>,
    session_api: Box<crate::apis::SessionApi>,
    user_api: Box<crate::apis::UserApi>,
    user_factor_api: Box<crate::apis::UserFactorApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            application_api: Box::new(crate::apis::ApplicationApiClient::new(rc.clone())),
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
