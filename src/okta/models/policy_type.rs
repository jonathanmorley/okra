#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#PolicyType {
    #[serde(rename = "OAUTH_AUTHORIZATION_POLICY")]
    r#OauthAuthorizationPolicy,
    #[serde(rename = "OKTA_SIGN_ON")]
    r#OktaSignOn,
    #[serde(rename = "PASSWORD")]
    r#Password,
}

impl Default for r#PolicyType {
    fn default() -> Self { r#PolicyType::OauthAuthorizationPolicy }
}
