#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#OAuthResponseType {
    #[serde(rename = "code")]
    r#Code,
    #[serde(rename = "token")]
    r#Token,
    #[serde(rename = "id_token")]
    r#IdToken,
}

impl Default for r#OAuthResponseType {
    fn default() -> Self { r#OAuthResponseType::Code }
}
