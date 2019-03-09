#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#AuthenticationProviderType {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    r#ActiveDirectory,
    #[serde(rename = "FEDERATION")]
    r#Federation,
    #[serde(rename = "LDAP")]
    r#Ldap,
    #[serde(rename = "OKTA")]
    r#Okta,
    #[serde(rename = "SOCIAL")]
    r#Social,
    #[serde(rename = "IMPORT")]
    r#Import,
}

impl Default for r#AuthenticationProviderType {
    fn default() -> Self { r#AuthenticationProviderType::ActiveDirectory }
}
