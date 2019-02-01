use std::borrow::Borrow;

use futures::Future;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::{configuration, Error};

pub struct PolicyApiClient {
    configuration: configuration::Configuration,
}

impl PolicyApiClient {
    pub fn new(configuration: configuration::Configuration) -> PolicyApiClient {
        PolicyApiClient {
            configuration: configuration,
        }
    }
}

pub trait PolicyApi {
    fn activate_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn activate_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn add_policy_rule(
        &self,
        policy_id: &str,
        policy_rule: crate::models::PolicyRule,
        activate: bool,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>>;
    fn create_policy(
        &self,
        policy: crate::models::Policy,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>>;
    fn deactivate_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn deactivate_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_policy(
        &self,
        policy_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>>;
    fn get_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>>;
    fn list_policies(
        &self,
        _type: &str,
        status: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Policy>, Error = Error<serde_json::Value>>>;
    fn list_policy_rules(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::PolicyRule>, Error = Error<serde_json::Value>>>;
    fn update_policy(
        &self,
        policy_id: &str,
        policy: crate::models::Policy,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>>;
    fn update_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
        policy_rule: crate::models::PolicyRule,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>>;
}

impl PolicyApi for PolicyApiClient {
    fn activate_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn activate_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/activate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn add_policy_rule(
        &self,
        policy_id: &str,
        policy_rule: crate::models::PolicyRule,
        activate: bool,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("activate".to_string(), activate.to_string())
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_body_param(policy_rule)
        .execute(self.configuration.borrow())
    }

    fn create_policy(
        &self,
        policy: crate::models::Policy,
        activate: bool,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::POST, "/api/v1/policies".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("activate".to_string(), activate.to_string())
            .with_body_param(policy)
            .execute(self.configuration.borrow())
    }

    fn deactivate_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/lifecycle/deactivate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn deactivate_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/deactivate".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_policy(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn delete_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::DELETE,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    fn get_policy(
        &self,
        policy_id: &str,
        expand: &str,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_query_param("expand".to_string(), expand.to_string())
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn get_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn list_policies(
        &self,
        _type: &str,
        status: &str,
        after: &str,
        limit: i32,
        expand: &str,
    ) -> Box<Future<Item = Vec<crate::models::Policy>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(hyper::Method::GET, "/api/v1/policies".to_string())
            .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
            .with_query_param("type".to_string(), _type.to_string())
            .with_query_param("status".to_string(), status.to_string())
            .with_query_param("after".to_string(), after.to_string())
            .with_query_param("limit".to_string(), limit.to_string())
            .with_query_param("expand".to_string(), expand.to_string())
            .execute(self.configuration.borrow())
    }

    fn list_policy_rules(
        &self,
        policy_id: &str,
    ) -> Box<Future<Item = Vec<crate::models::PolicyRule>, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/policies/{policyId}/rules".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .execute(self.configuration.borrow())
    }

    fn update_policy(
        &self,
        policy_id: &str,
        policy: crate::models::Policy,
    ) -> Box<Future<Item = crate::models::Policy, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/policies/{policyId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_body_param(policy)
        .execute(self.configuration.borrow())
    }

    fn update_policy_rule(
        &self,
        policy_id: &str,
        rule_id: &str,
        policy_rule: crate::models::PolicyRule,
    ) -> Box<Future<Item = crate::models::PolicyRule, Error = Error<serde_json::Value>>> {
        _internal_request::Request::new(
            hyper::Method::PUT,
            "/api/v1/policies/{policyId}/rules/{ruleId}".to_string(),
        )
        .with_auth(_internal_request::Auth::ApiKey(_internal_request::ApiKey {
            in_header: true,
            in_query: false,
            param_name: "Authorization".to_owned(),
        }))
        .with_path_param("policyId".to_string(), policy_id.to_string())
        .with_path_param("ruleId".to_string(), rule_id.to_string())
        .with_body_param(policy_rule)
        .execute(self.configuration.borrow())
    }
}
