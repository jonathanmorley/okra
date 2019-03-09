#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#UserStatus {
    #[serde(rename = "STAGED")]
    r#Staged,
    #[serde(rename = "PROVISIONED")]
    r#Provisioned,
    #[serde(rename = "ACTIVE")]
    r#Active,
    #[serde(rename = "RECOVERY")]
    r#Recovery,
    #[serde(rename = "PASSWORD_EXPIRED")]
    r#PasswordExpired,
    #[serde(rename = "LOCKED_OUT")]
    r#LockedOut,
    #[serde(rename = "DEPROVISIONED")]
    r#Deprovisioned,
    #[serde(rename = "SUSPENDED")]
    r#Suspended,
}

impl Default for r#UserStatus {
    fn default() -> Self { r#UserStatus::Staged }
}
