#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#LogCredentialProvider {
    #[serde(rename = "OKTA_AUTHENTICATION_PROVIDER")]
    r#OktaAuthenticationProvider,
    #[serde(rename = "RSA")]
    r#Rsa,
    #[serde(rename = "SYMANTEC")]
    r#Symantec,
    #[serde(rename = "GOOGLE")]
    r#Google,
    #[serde(rename = "DUO")]
    r#Duo,
    #[serde(rename = "YUBIKEY")]
    r#Yubikey,
}

impl Default for r#LogCredentialProvider {
    fn default() -> Self { r#LogCredentialProvider::OktaAuthenticationProvider }
}
