use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
  configuration: Rc<Configuration>,
  application_api: Box<::apis::ApplicationApi>,
  authentication_api: Box<::apis::AuthenticationApi>,
  group_api: Box<::apis::GroupApi>,
  log_api: Box<::apis::LogApi>,
  policy_api: Box<::apis::PolicyApi>,
  session_api: Box<::apis::SessionApi>,
  user_api: Box<::apis::UserApi>,
  user_factor_api: Box<::apis::UserFactorApi>,
}

impl APIClient {
  pub fn new(configuration: Configuration) -> APIClient {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      application_api: Box::new(::apis::ApplicationApiClient::new(rc.clone())),
      authentication_api: Box::new(::apis::AuthenticationApiClient::new(rc.clone())),
      group_api: Box::new(::apis::GroupApiClient::new(rc.clone())),
      log_api: Box::new(::apis::LogApiClient::new(rc.clone())),
      policy_api: Box::new(::apis::PolicyApiClient::new(rc.clone())),
      session_api: Box::new(::apis::SessionApiClient::new(rc.clone())),
      user_api: Box::new(::apis::UserApiClient::new(rc.clone())),
      user_factor_api: Box::new(::apis::UserFactorApiClient::new(rc.clone())),
    }
  }

  pub fn application_api(&self) -> &::apis::ApplicationApi{
    self.application_api.as_ref()
  }

  pub fn authentication_api(&self) -> &::apis::AuthenticationApi{
    self.authentication_api.as_ref()
  }

  pub fn group_api(&self) -> &::apis::GroupApi{
    self.group_api.as_ref()
  }

  pub fn log_api(&self) -> &::apis::LogApi{
    self.log_api.as_ref()
  }

  pub fn policy_api(&self) -> &::apis::PolicyApi{
    self.policy_api.as_ref()
  }

  pub fn session_api(&self) -> &::apis::SessionApi{
    self.session_api.as_ref()
  }

  pub fn user_api(&self) -> &::apis::UserApi{
    self.user_api.as_ref()
  }

  pub fn user_factor_api(&self) -> &::apis::UserFactorApi{
    self.user_factor_api.as_ref()
  }


}
