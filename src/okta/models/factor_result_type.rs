#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#FactorResultType {
    #[serde(rename = "SUCCESS")]
    r#Success,
    #[serde(rename = "CHALLENGE")]
    r#Challenge,
    #[serde(rename = "CANCELLED")]
    r#Cancelled,
    #[serde(rename = "WAITING")]
    r#Waiting,
    #[serde(rename = "FAILED")]
    r#Failed,
    #[serde(rename = "REJECTED")]
    r#Rejected,
    #[serde(rename = "TIMEOUT")]
    r#Timeout,
    #[serde(rename = "TIME_WINDOW_EXCEEDED")]
    r#TimeWindowExceeded,
    #[serde(rename = "PASSCODE_REPLAYED")]
    r#PasscodeReplayed,
    #[serde(rename = "ERROR")]
    r#Error,
}

impl Default for r#FactorResultType {
    fn default() -> Self { r#FactorResultType::Success }
}
