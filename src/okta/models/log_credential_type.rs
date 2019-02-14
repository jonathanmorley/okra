#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#LogCredentialType {
    #[serde(rename = "OTP")]
    r#Otp,
    #[serde(rename = "SMS")]
    r#Sms,
    #[serde(rename = "PASSWORD")]
    r#Password,
    #[serde(rename = "ASSERTION")]
    r#Assertion,
    #[serde(rename = "IWA")]
    r#Iwa,
    #[serde(rename = "EMAIL")]
    r#Email,
    #[serde(rename = "OAUTH2")]
    r#Oauth2,
    #[serde(rename = "JWT")]
    r#Jwt,
}

impl Default for r#LogCredentialType {
    fn default() -> Self { r#LogCredentialType::Otp }
}
