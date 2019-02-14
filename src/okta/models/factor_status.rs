#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#FactorStatus {
    #[serde(rename = "PENDING_ACTIVATION")]
    r#PendingActivation,
    #[serde(rename = "ACTIVE")]
    r#Active,
    #[serde(rename = "INACTIVE")]
    r#Inactive,
    #[serde(rename = "NOT_SETUP")]
    r#NotSetup,
    #[serde(rename = "ENROLLED")]
    r#Enrolled,
    #[serde(rename = "DISABLED")]
    r#Disabled,
    #[serde(rename = "EXPIRED")]
    r#Expired,
}

impl Default for r#FactorStatus {
    fn default() -> Self { r#FactorStatus::PendingActivation }
}
