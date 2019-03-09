#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#SamlApplicationSettingsSignOn {
    #[serde(rename = "assertionSigned", skip_serializing_if = "Option::is_none")]
    r#assertion_signed: Option<bool>,
    #[serde(rename = "attributeStatements", skip_serializing_if = "Option::is_none")]
    r#attribute_statements: Option<Vec<SamlAttributeStatement>>,
    #[serde(rename = "audience", skip_serializing_if = "Option::is_none")]
    r#audience: Option<String>,
    #[serde(rename = "audienceOverride", skip_serializing_if = "Option::is_none")]
    r#audience_override: Option<String>,
    #[serde(rename = "authnContextClassRef", skip_serializing_if = "Option::is_none")]
    r#authn_context_class_ref: Option<String>,
    #[serde(rename = "defaultRelayState", skip_serializing_if = "Option::is_none")]
    r#default_relay_state: Option<String>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    r#destination: Option<String>,
    #[serde(rename = "destinationOverride", skip_serializing_if = "Option::is_none")]
    r#destination_override: Option<String>,
    #[serde(rename = "digestAlgorithm", skip_serializing_if = "Option::is_none")]
    r#digest_algorithm: Option<String>,
    #[serde(rename = "honorForceAuthn", skip_serializing_if = "Option::is_none")]
    r#honor_force_authn: Option<bool>,
    #[serde(rename = "idpIssuer", skip_serializing_if = "Option::is_none")]
    r#idp_issuer: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    r#recipient: Option<String>,
    #[serde(rename = "recipientOverride", skip_serializing_if = "Option::is_none")]
    r#recipient_override: Option<String>,
    #[serde(rename = "requestCompressed", skip_serializing_if = "Option::is_none")]
    r#request_compressed: Option<bool>,
    #[serde(rename = "responseSigned", skip_serializing_if = "Option::is_none")]
    r#response_signed: Option<bool>,
    #[serde(rename = "signatureAlgorithm", skip_serializing_if = "Option::is_none")]
    r#signature_algorithm: Option<String>,
    #[serde(rename = "spIssuer", skip_serializing_if = "Option::is_none")]
    r#sp_issuer: Option<String>,
    #[serde(rename = "ssoAcsUrl", skip_serializing_if = "Option::is_none")]
    r#sso_acs_url: Option<String>,
    #[serde(rename = "ssoAcsUrlOverride", skip_serializing_if = "Option::is_none")]
    r#sso_acs_url_override: Option<String>,
    #[serde(rename = "subjectNameIdFormat", skip_serializing_if = "Option::is_none")]
    r#subject_name_id_format: Option<String>,
    #[serde(rename = "subjectNameIdTemplate", skip_serializing_if = "Option::is_none")]
    r#subject_name_id_template: Option<String>,
}

impl r#SamlApplicationSettingsSignOn {
    pub fn new(
    ) -> Self {
        Self {
          r#assertion_signed: None,
          r#attribute_statements: None,
          r#audience: None,
          r#audience_override: None,
          r#authn_context_class_ref: None,
          r#default_relay_state: None,
          r#destination: None,
          r#destination_override: None,
          r#digest_algorithm: None,
          r#honor_force_authn: None,
          r#idp_issuer: None,
          r#recipient: None,
          r#recipient_override: None,
          r#request_compressed: None,
          r#response_signed: None,
          r#signature_algorithm: None,
          r#sp_issuer: None,
          r#sso_acs_url: None,
          r#sso_acs_url_override: None,
          r#subject_name_id_format: None,
          r#subject_name_id_template: None,
        }
    }

    pub fn set_assertion_signed(&mut self, r#assertion_signed: bool) {
        self.r#assertion_signed = Some(r#assertion_signed);
    }

    pub fn with_assertion_signed(mut self, r#assertion_signed: bool) -> Self {
        self.r#assertion_signed = Some(r#assertion_signed);
        self
    }

    pub fn r#assertion_signed(&self) -> Option<&bool> {
        self.r#assertion_signed.as_ref().map(|x| x.borrow())
    }

    pub fn reset_assertion_signed(&mut self) {
        self.r#assertion_signed = None;
    }

    pub fn set_attribute_statements(&mut self, r#attribute_statements: Vec<SamlAttributeStatement>) {
        self.r#attribute_statements = Some(r#attribute_statements);
    }

    pub fn with_attribute_statements(mut self, r#attribute_statements: Vec<SamlAttributeStatement>) -> Self {
        self.r#attribute_statements = Some(r#attribute_statements);
        self
    }

    pub fn r#attribute_statements(&self) -> Option<&Vec<SamlAttributeStatement>> {
        self.r#attribute_statements.as_ref().map(|x| x.borrow())
    }

    pub fn reset_attribute_statements(&mut self) {
        self.r#attribute_statements = None;
    }

    pub fn set_audience(&mut self, r#audience: String) {
        self.r#audience = Some(r#audience);
    }

    pub fn with_audience(mut self, r#audience: String) -> Self {
        self.r#audience = Some(r#audience);
        self
    }

    pub fn r#audience(&self) -> Option<&str> {
        self.r#audience.as_ref().map(|x| x.borrow())
    }

    pub fn reset_audience(&mut self) {
        self.r#audience = None;
    }

    pub fn set_audience_override(&mut self, r#audience_override: String) {
        self.r#audience_override = Some(r#audience_override);
    }

    pub fn with_audience_override(mut self, r#audience_override: String) -> Self {
        self.r#audience_override = Some(r#audience_override);
        self
    }

    pub fn r#audience_override(&self) -> Option<&str> {
        self.r#audience_override.as_ref().map(|x| x.borrow())
    }

    pub fn reset_audience_override(&mut self) {
        self.r#audience_override = None;
    }

    pub fn set_authn_context_class_ref(&mut self, r#authn_context_class_ref: String) {
        self.r#authn_context_class_ref = Some(r#authn_context_class_ref);
    }

    pub fn with_authn_context_class_ref(mut self, r#authn_context_class_ref: String) -> Self {
        self.r#authn_context_class_ref = Some(r#authn_context_class_ref);
        self
    }

    pub fn r#authn_context_class_ref(&self) -> Option<&str> {
        self.r#authn_context_class_ref.as_ref().map(|x| x.borrow())
    }

    pub fn reset_authn_context_class_ref(&mut self) {
        self.r#authn_context_class_ref = None;
    }

    pub fn set_default_relay_state(&mut self, r#default_relay_state: String) {
        self.r#default_relay_state = Some(r#default_relay_state);
    }

    pub fn with_default_relay_state(mut self, r#default_relay_state: String) -> Self {
        self.r#default_relay_state = Some(r#default_relay_state);
        self
    }

    pub fn r#default_relay_state(&self) -> Option<&str> {
        self.r#default_relay_state.as_ref().map(|x| x.borrow())
    }

    pub fn reset_default_relay_state(&mut self) {
        self.r#default_relay_state = None;
    }

    pub fn set_destination(&mut self, r#destination: String) {
        self.r#destination = Some(r#destination);
    }

    pub fn with_destination(mut self, r#destination: String) -> Self {
        self.r#destination = Some(r#destination);
        self
    }

    pub fn r#destination(&self) -> Option<&str> {
        self.r#destination.as_ref().map(|x| x.borrow())
    }

    pub fn reset_destination(&mut self) {
        self.r#destination = None;
    }

    pub fn set_destination_override(&mut self, r#destination_override: String) {
        self.r#destination_override = Some(r#destination_override);
    }

    pub fn with_destination_override(mut self, r#destination_override: String) -> Self {
        self.r#destination_override = Some(r#destination_override);
        self
    }

    pub fn r#destination_override(&self) -> Option<&str> {
        self.r#destination_override.as_ref().map(|x| x.borrow())
    }

    pub fn reset_destination_override(&mut self) {
        self.r#destination_override = None;
    }

    pub fn set_digest_algorithm(&mut self, r#digest_algorithm: String) {
        self.r#digest_algorithm = Some(r#digest_algorithm);
    }

    pub fn with_digest_algorithm(mut self, r#digest_algorithm: String) -> Self {
        self.r#digest_algorithm = Some(r#digest_algorithm);
        self
    }

    pub fn r#digest_algorithm(&self) -> Option<&str> {
        self.r#digest_algorithm.as_ref().map(|x| x.borrow())
    }

    pub fn reset_digest_algorithm(&mut self) {
        self.r#digest_algorithm = None;
    }

    pub fn set_honor_force_authn(&mut self, r#honor_force_authn: bool) {
        self.r#honor_force_authn = Some(r#honor_force_authn);
    }

    pub fn with_honor_force_authn(mut self, r#honor_force_authn: bool) -> Self {
        self.r#honor_force_authn = Some(r#honor_force_authn);
        self
    }

    pub fn r#honor_force_authn(&self) -> Option<&bool> {
        self.r#honor_force_authn.as_ref().map(|x| x.borrow())
    }

    pub fn reset_honor_force_authn(&mut self) {
        self.r#honor_force_authn = None;
    }

    pub fn set_idp_issuer(&mut self, r#idp_issuer: String) {
        self.r#idp_issuer = Some(r#idp_issuer);
    }

    pub fn with_idp_issuer(mut self, r#idp_issuer: String) -> Self {
        self.r#idp_issuer = Some(r#idp_issuer);
        self
    }

    pub fn r#idp_issuer(&self) -> Option<&str> {
        self.r#idp_issuer.as_ref().map(|x| x.borrow())
    }

    pub fn reset_idp_issuer(&mut self) {
        self.r#idp_issuer = None;
    }

    pub fn set_recipient(&mut self, r#recipient: String) {
        self.r#recipient = Some(r#recipient);
    }

    pub fn with_recipient(mut self, r#recipient: String) -> Self {
        self.r#recipient = Some(r#recipient);
        self
    }

    pub fn r#recipient(&self) -> Option<&str> {
        self.r#recipient.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recipient(&mut self) {
        self.r#recipient = None;
    }

    pub fn set_recipient_override(&mut self, r#recipient_override: String) {
        self.r#recipient_override = Some(r#recipient_override);
    }

    pub fn with_recipient_override(mut self, r#recipient_override: String) -> Self {
        self.r#recipient_override = Some(r#recipient_override);
        self
    }

    pub fn r#recipient_override(&self) -> Option<&str> {
        self.r#recipient_override.as_ref().map(|x| x.borrow())
    }

    pub fn reset_recipient_override(&mut self) {
        self.r#recipient_override = None;
    }

    pub fn set_request_compressed(&mut self, r#request_compressed: bool) {
        self.r#request_compressed = Some(r#request_compressed);
    }

    pub fn with_request_compressed(mut self, r#request_compressed: bool) -> Self {
        self.r#request_compressed = Some(r#request_compressed);
        self
    }

    pub fn r#request_compressed(&self) -> Option<&bool> {
        self.r#request_compressed.as_ref().map(|x| x.borrow())
    }

    pub fn reset_request_compressed(&mut self) {
        self.r#request_compressed = None;
    }

    pub fn set_response_signed(&mut self, r#response_signed: bool) {
        self.r#response_signed = Some(r#response_signed);
    }

    pub fn with_response_signed(mut self, r#response_signed: bool) -> Self {
        self.r#response_signed = Some(r#response_signed);
        self
    }

    pub fn r#response_signed(&self) -> Option<&bool> {
        self.r#response_signed.as_ref().map(|x| x.borrow())
    }

    pub fn reset_response_signed(&mut self) {
        self.r#response_signed = None;
    }

    pub fn set_signature_algorithm(&mut self, r#signature_algorithm: String) {
        self.r#signature_algorithm = Some(r#signature_algorithm);
    }

    pub fn with_signature_algorithm(mut self, r#signature_algorithm: String) -> Self {
        self.r#signature_algorithm = Some(r#signature_algorithm);
        self
    }

    pub fn r#signature_algorithm(&self) -> Option<&str> {
        self.r#signature_algorithm.as_ref().map(|x| x.borrow())
    }

    pub fn reset_signature_algorithm(&mut self) {
        self.r#signature_algorithm = None;
    }

    pub fn set_sp_issuer(&mut self, r#sp_issuer: String) {
        self.r#sp_issuer = Some(r#sp_issuer);
    }

    pub fn with_sp_issuer(mut self, r#sp_issuer: String) -> Self {
        self.r#sp_issuer = Some(r#sp_issuer);
        self
    }

    pub fn r#sp_issuer(&self) -> Option<&str> {
        self.r#sp_issuer.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sp_issuer(&mut self) {
        self.r#sp_issuer = None;
    }

    pub fn set_sso_acs_url(&mut self, r#sso_acs_url: String) {
        self.r#sso_acs_url = Some(r#sso_acs_url);
    }

    pub fn with_sso_acs_url(mut self, r#sso_acs_url: String) -> Self {
        self.r#sso_acs_url = Some(r#sso_acs_url);
        self
    }

    pub fn r#sso_acs_url(&self) -> Option<&str> {
        self.r#sso_acs_url.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sso_acs_url(&mut self) {
        self.r#sso_acs_url = None;
    }

    pub fn set_sso_acs_url_override(&mut self, r#sso_acs_url_override: String) {
        self.r#sso_acs_url_override = Some(r#sso_acs_url_override);
    }

    pub fn with_sso_acs_url_override(mut self, r#sso_acs_url_override: String) -> Self {
        self.r#sso_acs_url_override = Some(r#sso_acs_url_override);
        self
    }

    pub fn r#sso_acs_url_override(&self) -> Option<&str> {
        self.r#sso_acs_url_override.as_ref().map(|x| x.borrow())
    }

    pub fn reset_sso_acs_url_override(&mut self) {
        self.r#sso_acs_url_override = None;
    }

    pub fn set_subject_name_id_format(&mut self, r#subject_name_id_format: String) {
        self.r#subject_name_id_format = Some(r#subject_name_id_format);
    }

    pub fn with_subject_name_id_format(mut self, r#subject_name_id_format: String) -> Self {
        self.r#subject_name_id_format = Some(r#subject_name_id_format);
        self
    }

    pub fn r#subject_name_id_format(&self) -> Option<&str> {
        self.r#subject_name_id_format.as_ref().map(|x| x.borrow())
    }

    pub fn reset_subject_name_id_format(&mut self) {
        self.r#subject_name_id_format = None;
    }

    pub fn set_subject_name_id_template(&mut self, r#subject_name_id_template: String) {
        self.r#subject_name_id_template = Some(r#subject_name_id_template);
    }

    pub fn with_subject_name_id_template(mut self, r#subject_name_id_template: String) -> Self {
        self.r#subject_name_id_template = Some(r#subject_name_id_template);
        self
    }

    pub fn r#subject_name_id_template(&self) -> Option<&str> {
        self.r#subject_name_id_template.as_ref().map(|x| x.borrow())
    }

    pub fn reset_subject_name_id_template(&mut self) {
        self.r#subject_name_id_template = None;
    }
}
