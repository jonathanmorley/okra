#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#UserNextLogin {
    #[serde(rename = "changePassword")]
    r#ChangePassword,
}

impl Default for r#UserNextLogin {
    fn default() -> Self { r#UserNextLogin::ChangePassword }
}
