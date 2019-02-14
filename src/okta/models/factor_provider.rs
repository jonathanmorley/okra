#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#FactorProvider {
    #[serde(rename = "OKTA")]
    r#Okta,
    #[serde(rename = "RSA")]
    r#Rsa,
    #[serde(rename = "GOOGLE")]
    r#Google,
    #[serde(rename = "SYMANTEC")]
    r#Symantec,
    #[serde(rename = "DUO")]
    r#Duo,
    #[serde(rename = "YUBICO")]
    r#Yubico,
    #[serde(rename = "FIDO")]
    r#Fido,
}

impl Default for r#FactorProvider {
    fn default() -> Self { r#FactorProvider::Okta }
}
