#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#FactorType {
    #[serde(rename = "push")]
    r#Push,
    #[serde(rename = "sms")]
    r#Sms,
    #[serde(rename = "call")]
    r#Call,
    #[serde(rename = "token")]
    r#Token,
    #[serde(rename = "token:software:totp")]
    r#TokenSoftwareTotp,
    #[serde(rename = "token:hardware")]
    r#TokenHardware,
    #[serde(rename = "question")]
    r#Question,
    #[serde(rename = "web")]
    r#Web,
    #[serde(rename = "email")]
    r#Email,
    #[serde(rename = "u2f")]
    r#U2f,
    #[serde(rename = "webauthn")]
    r#Webauthn,
    #[serde(rename = "token:software")]
    r#TokenSoftware,
    #[serde(rename = "custom")]
    r#Custom,
}

impl Default for r#FactorType {
    fn default() -> Self { r#FactorType::Push }
}
