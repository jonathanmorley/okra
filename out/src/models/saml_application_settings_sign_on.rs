/* 
 * Okta API
 *
 * Allows customers to easily access the Okta API
 *
 * OpenAPI spec version: 1.9.0
 * Contact: devex-public@okta.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SamlApplicationSettingsSignOn {
  #[serde(rename = "assertionSigned")]
  assertion_signed: Option<bool>,
  #[serde(rename = "attributeStatements")]
  attribute_statements: Option<Vec<::models::SamlAttributeStatement>>,
  #[serde(rename = "audience")]
  audience: Option<String>,
  #[serde(rename = "audienceOverride")]
  audience_override: Option<String>,
  #[serde(rename = "authnContextClassRef")]
  authn_context_class_ref: Option<String>,
  #[serde(rename = "defaultRelayState")]
  default_relay_state: Option<String>,
  #[serde(rename = "destination")]
  destination: Option<String>,
  #[serde(rename = "destinationOverride")]
  destination_override: Option<String>,
  #[serde(rename = "digestAlgorithm")]
  digest_algorithm: Option<String>,
  #[serde(rename = "honorForceAuthn")]
  honor_force_authn: Option<bool>,
  #[serde(rename = "idpIssuer")]
  idp_issuer: Option<String>,
  #[serde(rename = "recipient")]
  recipient: Option<String>,
  #[serde(rename = "recipientOverride")]
  recipient_override: Option<String>,
  #[serde(rename = "requestCompressed")]
  request_compressed: Option<bool>,
  #[serde(rename = "responseSigned")]
  response_signed: Option<bool>,
  #[serde(rename = "signatureAlgorithm")]
  signature_algorithm: Option<String>,
  #[serde(rename = "spIssuer")]
  sp_issuer: Option<String>,
  #[serde(rename = "ssoAcsUrl")]
  sso_acs_url: Option<String>,
  #[serde(rename = "ssoAcsUrlOverride")]
  sso_acs_url_override: Option<String>,
  #[serde(rename = "subjectNameIdFormat")]
  subject_name_id_format: Option<String>,
  #[serde(rename = "subjectNameIdTemplate")]
  subject_name_id_template: Option<String>
}

impl SamlApplicationSettingsSignOn {
  pub fn new() -> SamlApplicationSettingsSignOn {
    SamlApplicationSettingsSignOn {
      assertion_signed: None,
      attribute_statements: None,
      audience: None,
      audience_override: None,
      authn_context_class_ref: None,
      default_relay_state: None,
      destination: None,
      destination_override: None,
      digest_algorithm: None,
      honor_force_authn: None,
      idp_issuer: None,
      recipient: None,
      recipient_override: None,
      request_compressed: None,
      response_signed: None,
      signature_algorithm: None,
      sp_issuer: None,
      sso_acs_url: None,
      sso_acs_url_override: None,
      subject_name_id_format: None,
      subject_name_id_template: None
    }
  }

  pub fn set_assertion_signed(&mut self, assertion_signed: bool) {
    self.assertion_signed = Some(assertion_signed);
  }

  pub fn with_assertion_signed(mut self, assertion_signed: bool) -> SamlApplicationSettingsSignOn {
    self.assertion_signed = Some(assertion_signed);
    self
  }

  pub fn assertion_signed(&self) -> Option<&bool> {
    self.assertion_signed.as_ref()
  }

  pub fn reset_assertion_signed(&mut self) {
    self.assertion_signed = None;
  }

  pub fn set_attribute_statements(&mut self, attribute_statements: Vec<::models::SamlAttributeStatement>) {
    self.attribute_statements = Some(attribute_statements);
  }

  pub fn with_attribute_statements(mut self, attribute_statements: Vec<::models::SamlAttributeStatement>) -> SamlApplicationSettingsSignOn {
    self.attribute_statements = Some(attribute_statements);
    self
  }

  pub fn attribute_statements(&self) -> Option<&Vec<::models::SamlAttributeStatement>> {
    self.attribute_statements.as_ref()
  }

  pub fn reset_attribute_statements(&mut self) {
    self.attribute_statements = None;
  }

  pub fn set_audience(&mut self, audience: String) {
    self.audience = Some(audience);
  }

  pub fn with_audience(mut self, audience: String) -> SamlApplicationSettingsSignOn {
    self.audience = Some(audience);
    self
  }

  pub fn audience(&self) -> Option<&String> {
    self.audience.as_ref()
  }

  pub fn reset_audience(&mut self) {
    self.audience = None;
  }

  pub fn set_audience_override(&mut self, audience_override: String) {
    self.audience_override = Some(audience_override);
  }

  pub fn with_audience_override(mut self, audience_override: String) -> SamlApplicationSettingsSignOn {
    self.audience_override = Some(audience_override);
    self
  }

  pub fn audience_override(&self) -> Option<&String> {
    self.audience_override.as_ref()
  }

  pub fn reset_audience_override(&mut self) {
    self.audience_override = None;
  }

  pub fn set_authn_context_class_ref(&mut self, authn_context_class_ref: String) {
    self.authn_context_class_ref = Some(authn_context_class_ref);
  }

  pub fn with_authn_context_class_ref(mut self, authn_context_class_ref: String) -> SamlApplicationSettingsSignOn {
    self.authn_context_class_ref = Some(authn_context_class_ref);
    self
  }

  pub fn authn_context_class_ref(&self) -> Option<&String> {
    self.authn_context_class_ref.as_ref()
  }

  pub fn reset_authn_context_class_ref(&mut self) {
    self.authn_context_class_ref = None;
  }

  pub fn set_default_relay_state(&mut self, default_relay_state: String) {
    self.default_relay_state = Some(default_relay_state);
  }

  pub fn with_default_relay_state(mut self, default_relay_state: String) -> SamlApplicationSettingsSignOn {
    self.default_relay_state = Some(default_relay_state);
    self
  }

  pub fn default_relay_state(&self) -> Option<&String> {
    self.default_relay_state.as_ref()
  }

  pub fn reset_default_relay_state(&mut self) {
    self.default_relay_state = None;
  }

  pub fn set_destination(&mut self, destination: String) {
    self.destination = Some(destination);
  }

  pub fn with_destination(mut self, destination: String) -> SamlApplicationSettingsSignOn {
    self.destination = Some(destination);
    self
  }

  pub fn destination(&self) -> Option<&String> {
    self.destination.as_ref()
  }

  pub fn reset_destination(&mut self) {
    self.destination = None;
  }

  pub fn set_destination_override(&mut self, destination_override: String) {
    self.destination_override = Some(destination_override);
  }

  pub fn with_destination_override(mut self, destination_override: String) -> SamlApplicationSettingsSignOn {
    self.destination_override = Some(destination_override);
    self
  }

  pub fn destination_override(&self) -> Option<&String> {
    self.destination_override.as_ref()
  }

  pub fn reset_destination_override(&mut self) {
    self.destination_override = None;
  }

  pub fn set_digest_algorithm(&mut self, digest_algorithm: String) {
    self.digest_algorithm = Some(digest_algorithm);
  }

  pub fn with_digest_algorithm(mut self, digest_algorithm: String) -> SamlApplicationSettingsSignOn {
    self.digest_algorithm = Some(digest_algorithm);
    self
  }

  pub fn digest_algorithm(&self) -> Option<&String> {
    self.digest_algorithm.as_ref()
  }

  pub fn reset_digest_algorithm(&mut self) {
    self.digest_algorithm = None;
  }

  pub fn set_honor_force_authn(&mut self, honor_force_authn: bool) {
    self.honor_force_authn = Some(honor_force_authn);
  }

  pub fn with_honor_force_authn(mut self, honor_force_authn: bool) -> SamlApplicationSettingsSignOn {
    self.honor_force_authn = Some(honor_force_authn);
    self
  }

  pub fn honor_force_authn(&self) -> Option<&bool> {
    self.honor_force_authn.as_ref()
  }

  pub fn reset_honor_force_authn(&mut self) {
    self.honor_force_authn = None;
  }

  pub fn set_idp_issuer(&mut self, idp_issuer: String) {
    self.idp_issuer = Some(idp_issuer);
  }

  pub fn with_idp_issuer(mut self, idp_issuer: String) -> SamlApplicationSettingsSignOn {
    self.idp_issuer = Some(idp_issuer);
    self
  }

  pub fn idp_issuer(&self) -> Option<&String> {
    self.idp_issuer.as_ref()
  }

  pub fn reset_idp_issuer(&mut self) {
    self.idp_issuer = None;
  }

  pub fn set_recipient(&mut self, recipient: String) {
    self.recipient = Some(recipient);
  }

  pub fn with_recipient(mut self, recipient: String) -> SamlApplicationSettingsSignOn {
    self.recipient = Some(recipient);
    self
  }

  pub fn recipient(&self) -> Option<&String> {
    self.recipient.as_ref()
  }

  pub fn reset_recipient(&mut self) {
    self.recipient = None;
  }

  pub fn set_recipient_override(&mut self, recipient_override: String) {
    self.recipient_override = Some(recipient_override);
  }

  pub fn with_recipient_override(mut self, recipient_override: String) -> SamlApplicationSettingsSignOn {
    self.recipient_override = Some(recipient_override);
    self
  }

  pub fn recipient_override(&self) -> Option<&String> {
    self.recipient_override.as_ref()
  }

  pub fn reset_recipient_override(&mut self) {
    self.recipient_override = None;
  }

  pub fn set_request_compressed(&mut self, request_compressed: bool) {
    self.request_compressed = Some(request_compressed);
  }

  pub fn with_request_compressed(mut self, request_compressed: bool) -> SamlApplicationSettingsSignOn {
    self.request_compressed = Some(request_compressed);
    self
  }

  pub fn request_compressed(&self) -> Option<&bool> {
    self.request_compressed.as_ref()
  }

  pub fn reset_request_compressed(&mut self) {
    self.request_compressed = None;
  }

  pub fn set_response_signed(&mut self, response_signed: bool) {
    self.response_signed = Some(response_signed);
  }

  pub fn with_response_signed(mut self, response_signed: bool) -> SamlApplicationSettingsSignOn {
    self.response_signed = Some(response_signed);
    self
  }

  pub fn response_signed(&self) -> Option<&bool> {
    self.response_signed.as_ref()
  }

  pub fn reset_response_signed(&mut self) {
    self.response_signed = None;
  }

  pub fn set_signature_algorithm(&mut self, signature_algorithm: String) {
    self.signature_algorithm = Some(signature_algorithm);
  }

  pub fn with_signature_algorithm(mut self, signature_algorithm: String) -> SamlApplicationSettingsSignOn {
    self.signature_algorithm = Some(signature_algorithm);
    self
  }

  pub fn signature_algorithm(&self) -> Option<&String> {
    self.signature_algorithm.as_ref()
  }

  pub fn reset_signature_algorithm(&mut self) {
    self.signature_algorithm = None;
  }

  pub fn set_sp_issuer(&mut self, sp_issuer: String) {
    self.sp_issuer = Some(sp_issuer);
  }

  pub fn with_sp_issuer(mut self, sp_issuer: String) -> SamlApplicationSettingsSignOn {
    self.sp_issuer = Some(sp_issuer);
    self
  }

  pub fn sp_issuer(&self) -> Option<&String> {
    self.sp_issuer.as_ref()
  }

  pub fn reset_sp_issuer(&mut self) {
    self.sp_issuer = None;
  }

  pub fn set_sso_acs_url(&mut self, sso_acs_url: String) {
    self.sso_acs_url = Some(sso_acs_url);
  }

  pub fn with_sso_acs_url(mut self, sso_acs_url: String) -> SamlApplicationSettingsSignOn {
    self.sso_acs_url = Some(sso_acs_url);
    self
  }

  pub fn sso_acs_url(&self) -> Option<&String> {
    self.sso_acs_url.as_ref()
  }

  pub fn reset_sso_acs_url(&mut self) {
    self.sso_acs_url = None;
  }

  pub fn set_sso_acs_url_override(&mut self, sso_acs_url_override: String) {
    self.sso_acs_url_override = Some(sso_acs_url_override);
  }

  pub fn with_sso_acs_url_override(mut self, sso_acs_url_override: String) -> SamlApplicationSettingsSignOn {
    self.sso_acs_url_override = Some(sso_acs_url_override);
    self
  }

  pub fn sso_acs_url_override(&self) -> Option<&String> {
    self.sso_acs_url_override.as_ref()
  }

  pub fn reset_sso_acs_url_override(&mut self) {
    self.sso_acs_url_override = None;
  }

  pub fn set_subject_name_id_format(&mut self, subject_name_id_format: String) {
    self.subject_name_id_format = Some(subject_name_id_format);
  }

  pub fn with_subject_name_id_format(mut self, subject_name_id_format: String) -> SamlApplicationSettingsSignOn {
    self.subject_name_id_format = Some(subject_name_id_format);
    self
  }

  pub fn subject_name_id_format(&self) -> Option<&String> {
    self.subject_name_id_format.as_ref()
  }

  pub fn reset_subject_name_id_format(&mut self) {
    self.subject_name_id_format = None;
  }

  pub fn set_subject_name_id_template(&mut self, subject_name_id_template: String) {
    self.subject_name_id_template = Some(subject_name_id_template);
  }

  pub fn with_subject_name_id_template(mut self, subject_name_id_template: String) -> SamlApplicationSettingsSignOn {
    self.subject_name_id_template = Some(subject_name_id_template);
    self
  }

  pub fn subject_name_id_template(&self) -> Option<&String> {
    self.subject_name_id_template.as_ref()
  }

  pub fn reset_subject_name_id_template(&mut self) {
    self.subject_name_id_template = None;
  }

}


