#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#EmailStatus {
    #[serde(rename = "VERIFIED")]
    r#Verified,
    #[serde(rename = "UNVERIFIED")]
    r#Unverified,
}

impl Default for r#EmailStatus {
    fn default() -> Self { r#EmailStatus::Verified }
}
