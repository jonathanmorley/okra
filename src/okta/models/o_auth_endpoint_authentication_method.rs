#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#OAuthEndpointAuthenticationMethod {
    #[serde(rename = "none")]
    r#None,
    #[serde(rename = "client_secret_post")]
    r#ClientSecretPost,
    #[serde(rename = "client_secret_basic")]
    r#ClientSecretBasic,
    #[serde(rename = "client_secret_jwt")]
    r#ClientSecretJwt,
}

impl Default for r#OAuthEndpointAuthenticationMethod {
    fn default() -> Self { r#OAuthEndpointAuthenticationMethod::None }
}
