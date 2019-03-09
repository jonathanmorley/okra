#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#LogAuthenticationProvider {
    #[serde(rename = "OKTA_AUTHENTICATION_PROVIDER")]
    r#OktaAuthenticationProvider,
    #[serde(rename = "ACTIVE_DIRECTORY")]
    r#ActiveDirectory,
    #[serde(rename = "LDAP")]
    r#Ldap,
    #[serde(rename = "FEDERATION")]
    r#Federation,
    #[serde(rename = "SOCIAL")]
    r#Social,
    #[serde(rename = "FACTOR_PROVIDER")]
    r#FactorProvider,
}

impl Default for r#LogAuthenticationProvider {
    fn default() -> Self { r#LogAuthenticationProvider::OktaAuthenticationProvider }
}
