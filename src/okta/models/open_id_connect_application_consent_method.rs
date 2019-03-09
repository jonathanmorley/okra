#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#OpenIdConnectApplicationConsentMethod {
    #[serde(rename = "REQUIRED")]
    r#Required,
    #[serde(rename = "TRUSTED")]
    r#Trusted,
}

impl Default for r#OpenIdConnectApplicationConsentMethod {
    fn default() -> Self { r#OpenIdConnectApplicationConsentMethod::Required }
}
