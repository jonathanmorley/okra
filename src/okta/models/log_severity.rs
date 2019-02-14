#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#LogSeverity {
    #[serde(rename = "DEBUG")]
    r#Debug,
    #[serde(rename = "INFO")]
    r#Info,
    #[serde(rename = "WARN")]
    r#Warn,
    #[serde(rename = "ERROR")]
    r#Error,
}

impl Default for r#LogSeverity {
    fn default() -> Self { r#LogSeverity::Debug }
}
