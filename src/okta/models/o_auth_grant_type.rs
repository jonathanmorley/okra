#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#OAuthGrantType {
    #[serde(rename = "authorization_code")]
    r#AuthorizationCode,
    #[serde(rename = "implicit")]
    r#Implicit,
    #[serde(rename = "password")]
    r#Password,
    #[serde(rename = "refresh_token")]
    r#RefreshToken,
    #[serde(rename = "client_credentials")]
    r#ClientCredentials,
}

impl Default for r#OAuthGrantType {
    fn default() -> Self { r#OAuthGrantType::AuthorizationCode }
}
