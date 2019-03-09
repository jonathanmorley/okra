#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum r#ApplicationSignOnMode {
    #[serde(rename = "BOOKMARK")]
    r#Bookmark,
    #[serde(rename = "BASIC_AUTH")]
    r#BasicAuth,
    #[serde(rename = "BROWSER_PLUGIN")]
    r#BrowserPlugin,
    #[serde(rename = "SECURE_PASSWORD_STORE")]
    r#SecurePasswordStore,
    #[serde(rename = "AUTO_LOGIN")]
    r#AutoLogin,
    #[serde(rename = "WS_FEDERATION")]
    r#WsFederation,
    #[serde(rename = "SAML_2_0")]
    r#Saml20,
    #[serde(rename = "OPENID_CONNECT")]
    r#OpenidConnect,
    #[serde(rename = "SAML_1_1")]
    r#Saml11,
}

impl Default for r#ApplicationSignOnMode {
    fn default() -> Self { r#ApplicationSignOnMode::Bookmark }
}
