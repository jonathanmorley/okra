#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#RoleStatus {
    #[serde(rename = "ACTIVE")]
    r#Active,
    #[serde(rename = "INACTIVE")]
    r#Inactive,
}

impl Default for r#RoleStatus {
    fn default() -> Self { r#RoleStatus::Active }
}
