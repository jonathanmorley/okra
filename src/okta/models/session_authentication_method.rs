#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#SessionAuthenticationMethod {
    #[serde(rename = "pwd")]
    r#Pwd,
    #[serde(rename = "swk")]
    r#Swk,
    #[serde(rename = "hwk")]
    r#Hwk,
    #[serde(rename = "otp")]
    r#Otp,
    #[serde(rename = "sms")]
    r#Sms,
    #[serde(rename = "tel")]
    r#Tel,
    #[serde(rename = "geo")]
    r#Geo,
    #[serde(rename = "fpt")]
    r#Fpt,
    #[serde(rename = "kba")]
    r#Kba,
    #[serde(rename = "mfa")]
    r#Mfa,
}

impl Default for r#SessionAuthenticationMethod {
    fn default() -> Self { r#SessionAuthenticationMethod::Pwd }
}
