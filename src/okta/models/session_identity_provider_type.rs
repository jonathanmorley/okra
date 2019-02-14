#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#SessionIdentityProviderType {
    #[serde(rename = "ACTIVE_DIRECTORY")]
    r#ActiveDirectory,
    #[serde(rename = "LDAP")]
    r#Ldap,
    #[serde(rename = "OKTA")]
    r#Okta,
    #[serde(rename = "FEDERATION")]
    r#Federation,
    #[serde(rename = "SOCIAL")]
    r#Social,
}

impl Default for r#SessionIdentityProviderType {
    fn default() -> Self { r#SessionIdentityProviderType::ActiveDirectory }
}
