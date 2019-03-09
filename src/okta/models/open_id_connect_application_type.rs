#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#OpenIdConnectApplicationType {
    #[serde(rename = "web")]
    r#Web,
    #[serde(rename = "native")]
    r#Native,
    #[serde(rename = "browser")]
    r#Browser,
    #[serde(rename = "service")]
    r#Service,
}

impl Default for r#OpenIdConnectApplicationType {
    fn default() -> Self { r#OpenIdConnectApplicationType::Web }
}
