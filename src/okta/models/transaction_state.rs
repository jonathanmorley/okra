#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#TransactionState {
    #[serde(rename = "UNAUTHENTICATED")]
    r#Unauthenticated,
    #[serde(rename = "PASSWORD_WARN")]
    r#PasswordWarn,
    #[serde(rename = "PASSWORD_EXPIRED")]
    r#PasswordExpired,
    #[serde(rename = "RECOVERY")]
    r#Recovery,
    #[serde(rename = "RECOVERY_CHALLENGE")]
    r#RecoveryChallenge,
    #[serde(rename = "PASSWORD_RESET")]
    r#PasswordReset,
    #[serde(rename = "LOCKED_OUT")]
    r#LockedOut,
    #[serde(rename = "MFA_ENROLL")]
    r#MfaEnroll,
    #[serde(rename = "MFA_ENROLL_ACTIVATE")]
    r#MfaEnrollActivate,
    #[serde(rename = "MFA_REQUIRED")]
    r#MfaRequired,
    #[serde(rename = "MFA_CHALLENGE")]
    r#MfaChallenge,
    #[serde(rename = "SUCCESS")]
    r#Success,
}

impl Default for r#TransactionState {
    fn default() -> Self { r#TransactionState::Unauthenticated }
}
