#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#ApplicationCredentialsScheme {
    #[serde(rename = "SHARED_USERNAME_AND_PASSWORD")]
    r#SharedUsernameAndPassword,
    #[serde(rename = "EXTERNAL_PASSWORD_SYNC")]
    r#ExternalPasswordSync,
    #[serde(rename = "EDIT_USERNAME_AND_PASSWORD")]
    r#EditUsernameAndPassword,
    #[serde(rename = "EDIT_PASSWORD_ONLY")]
    r#EditPasswordOnly,
    #[serde(rename = "ADMIN_SETS_CREDENTIALS")]
    r#AdminSetsCredentials,
}

impl Default for r#ApplicationCredentialsScheme {
    fn default() -> Self { r#ApplicationCredentialsScheme::SharedUsernameAndPassword }
}
