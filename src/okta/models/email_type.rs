#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#EmailType {
    #[serde(rename = "PRIMARY")]
    r#Primary,
    #[serde(rename = "SECONDARY")]
    r#Secondary,
}

impl Default for r#EmailType {
    fn default() -> Self { r#EmailType::Primary }
}
