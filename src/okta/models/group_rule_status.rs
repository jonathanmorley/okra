#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#GroupRuleStatus {
    #[serde(rename = "ACTIVE")]
    r#Active,
    #[serde(rename = "INACTIVE")]
    r#Inactive,
    #[serde(rename = "INVALID")]
    r#Invalid,
}

impl Default for r#GroupRuleStatus {
    fn default() -> Self { r#GroupRuleStatus::Active }
}
