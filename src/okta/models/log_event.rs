#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogEvent {
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    r#actor: Option<LogActor>,
    #[serde(rename = "authenticationContext", skip_serializing_if = "Option::is_none")]
    r#authentication_context: Option<LogAuthenticationContext>,
    #[serde(rename = "client", skip_serializing_if = "Option::is_none")]
    r#client: Option<LogClient>,
    #[serde(rename = "debugContext", skip_serializing_if = "Option::is_none")]
    r#debug_context: Option<LogDebugContext>,
    #[serde(rename = "displayMessage", skip_serializing_if = "Option::is_none")]
    r#display_message: Option<String>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    r#event_type: Option<String>,
    #[serde(rename = "legacyEventType", skip_serializing_if = "Option::is_none")]
    r#legacy_event_type: Option<String>,
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    r#outcome: Option<LogOutcome>,
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    r#published: Option<String>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    r#request: Option<LogRequest>,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    r#security_context: Option<LogSecurityContext>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    r#severity: Option<LogSeverity>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    r#target: Option<Vec<LogTarget>>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    r#transaction: Option<LogTransaction>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    r#uuid: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    r#version: Option<String>,
}

impl r#LogEvent {
    pub fn new(
    ) -> Self {
        Self {
          r#actor: None,
          r#authentication_context: None,
          r#client: None,
          r#debug_context: None,
          r#display_message: None,
          r#event_type: None,
          r#legacy_event_type: None,
          r#outcome: None,
          r#published: None,
          r#request: None,
          r#security_context: None,
          r#severity: None,
          r#target: None,
          r#transaction: None,
          r#uuid: None,
          r#version: None,
        }
    }

    pub fn set_actor(&mut self, r#actor: LogActor) {
        self.r#actor = Some(r#actor);
    }

    pub fn with_actor(mut self, r#actor: LogActor) -> Self {
        self.r#actor = Some(r#actor);
        self
    }

    pub fn r#actor(&self) -> Option<&LogActor> {
        self.r#actor.as_ref().map(|x| x.borrow())
    }

    pub fn reset_actor(&mut self) {
        self.r#actor = None;
    }

    pub fn set_authentication_context(&mut self, r#authentication_context: LogAuthenticationContext) {
        self.r#authentication_context = Some(r#authentication_context);
    }

    pub fn with_authentication_context(mut self, r#authentication_context: LogAuthenticationContext) -> Self {
        self.r#authentication_context = Some(r#authentication_context);
        self
    }

    pub fn r#authentication_context(&self) -> Option<&LogAuthenticationContext> {
        self.r#authentication_context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_authentication_context(&mut self) {
        self.r#authentication_context = None;
    }

    pub fn set_client(&mut self, r#client: LogClient) {
        self.r#client = Some(r#client);
    }

    pub fn with_client(mut self, r#client: LogClient) -> Self {
        self.r#client = Some(r#client);
        self
    }

    pub fn r#client(&self) -> Option<&LogClient> {
        self.r#client.as_ref().map(|x| x.borrow())
    }

    pub fn reset_client(&mut self) {
        self.r#client = None;
    }

    pub fn set_debug_context(&mut self, r#debug_context: LogDebugContext) {
        self.r#debug_context = Some(r#debug_context);
    }

    pub fn with_debug_context(mut self, r#debug_context: LogDebugContext) -> Self {
        self.r#debug_context = Some(r#debug_context);
        self
    }

    pub fn r#debug_context(&self) -> Option<&LogDebugContext> {
        self.r#debug_context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_debug_context(&mut self) {
        self.r#debug_context = None;
    }

    pub fn set_display_message(&mut self, r#display_message: String) {
        self.r#display_message = Some(r#display_message);
    }

    pub fn with_display_message(mut self, r#display_message: String) -> Self {
        self.r#display_message = Some(r#display_message);
        self
    }

    pub fn r#display_message(&self) -> Option<&str> {
        self.r#display_message.as_ref().map(|x| x.borrow())
    }

    pub fn reset_display_message(&mut self) {
        self.r#display_message = None;
    }

    pub fn set_event_type(&mut self, r#event_type: String) {
        self.r#event_type = Some(r#event_type);
    }

    pub fn with_event_type(mut self, r#event_type: String) -> Self {
        self.r#event_type = Some(r#event_type);
        self
    }

    pub fn r#event_type(&self) -> Option<&str> {
        self.r#event_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_event_type(&mut self) {
        self.r#event_type = None;
    }

    pub fn set_legacy_event_type(&mut self, r#legacy_event_type: String) {
        self.r#legacy_event_type = Some(r#legacy_event_type);
    }

    pub fn with_legacy_event_type(mut self, r#legacy_event_type: String) -> Self {
        self.r#legacy_event_type = Some(r#legacy_event_type);
        self
    }

    pub fn r#legacy_event_type(&self) -> Option<&str> {
        self.r#legacy_event_type.as_ref().map(|x| x.borrow())
    }

    pub fn reset_legacy_event_type(&mut self) {
        self.r#legacy_event_type = None;
    }

    pub fn set_outcome(&mut self, r#outcome: LogOutcome) {
        self.r#outcome = Some(r#outcome);
    }

    pub fn with_outcome(mut self, r#outcome: LogOutcome) -> Self {
        self.r#outcome = Some(r#outcome);
        self
    }

    pub fn r#outcome(&self) -> Option<&LogOutcome> {
        self.r#outcome.as_ref().map(|x| x.borrow())
    }

    pub fn reset_outcome(&mut self) {
        self.r#outcome = None;
    }

    pub fn set_published(&mut self, r#published: String) {
        self.r#published = Some(r#published);
    }

    pub fn with_published(mut self, r#published: String) -> Self {
        self.r#published = Some(r#published);
        self
    }

    pub fn r#published(&self) -> Option<&str> {
        self.r#published.as_ref().map(|x| x.borrow())
    }

    pub fn reset_published(&mut self) {
        self.r#published = None;
    }

    pub fn set_request(&mut self, r#request: LogRequest) {
        self.r#request = Some(r#request);
    }

    pub fn with_request(mut self, r#request: LogRequest) -> Self {
        self.r#request = Some(r#request);
        self
    }

    pub fn r#request(&self) -> Option<&LogRequest> {
        self.r#request.as_ref().map(|x| x.borrow())
    }

    pub fn reset_request(&mut self) {
        self.r#request = None;
    }

    pub fn set_security_context(&mut self, r#security_context: LogSecurityContext) {
        self.r#security_context = Some(r#security_context);
    }

    pub fn with_security_context(mut self, r#security_context: LogSecurityContext) -> Self {
        self.r#security_context = Some(r#security_context);
        self
    }

    pub fn r#security_context(&self) -> Option<&LogSecurityContext> {
        self.r#security_context.as_ref().map(|x| x.borrow())
    }

    pub fn reset_security_context(&mut self) {
        self.r#security_context = None;
    }

    pub fn set_severity(&mut self, r#severity: LogSeverity) {
        self.r#severity = Some(r#severity);
    }

    pub fn with_severity(mut self, r#severity: LogSeverity) -> Self {
        self.r#severity = Some(r#severity);
        self
    }

    pub fn r#severity(&self) -> Option<&LogSeverity> {
        self.r#severity.as_ref().map(|x| x.borrow())
    }

    pub fn reset_severity(&mut self) {
        self.r#severity = None;
    }

    pub fn set_target(&mut self, r#target: Vec<LogTarget>) {
        self.r#target = Some(r#target);
    }

    pub fn with_target(mut self, r#target: Vec<LogTarget>) -> Self {
        self.r#target = Some(r#target);
        self
    }

    pub fn r#target(&self) -> Option<&Vec<LogTarget>> {
        self.r#target.as_ref().map(|x| x.borrow())
    }

    pub fn reset_target(&mut self) {
        self.r#target = None;
    }

    pub fn set_transaction(&mut self, r#transaction: LogTransaction) {
        self.r#transaction = Some(r#transaction);
    }

    pub fn with_transaction(mut self, r#transaction: LogTransaction) -> Self {
        self.r#transaction = Some(r#transaction);
        self
    }

    pub fn r#transaction(&self) -> Option<&LogTransaction> {
        self.r#transaction.as_ref().map(|x| x.borrow())
    }

    pub fn reset_transaction(&mut self) {
        self.r#transaction = None;
    }

    pub fn set_uuid(&mut self, r#uuid: String) {
        self.r#uuid = Some(r#uuid);
    }

    pub fn with_uuid(mut self, r#uuid: String) -> Self {
        self.r#uuid = Some(r#uuid);
        self
    }

    pub fn r#uuid(&self) -> Option<&str> {
        self.r#uuid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_uuid(&mut self) {
        self.r#uuid = None;
    }

    pub fn set_version(&mut self, r#version: String) {
        self.r#version = Some(r#version);
    }

    pub fn with_version(mut self, r#version: String) -> Self {
        self.r#version = Some(r#version);
        self
    }

    pub fn r#version(&self) -> Option<&str> {
        self.r#version.as_ref().map(|x| x.borrow())
    }

    pub fn reset_version(&mut self) {
        self.r#version = None;
    }
}
