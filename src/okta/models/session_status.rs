#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#SessionStatus {
    #[serde(rename = "ACTIVE")]
    r#Active,
    #[serde(rename = "MFA_ENROLL")]
    r#MfaEnroll,
    #[serde(rename = "MFA_REQUIRED")]
    r#MfaRequired,
}

impl Default for r#SessionStatus {
    fn default() -> Self { r#SessionStatus::Active }
}
