use std::collections::HashMap;

#[derive(Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    pub cookies: CookieJar,
    // TODO: take an oauth2 token source, similar to the Go one
}

pub type BasicAuth = (String, Option<String>);
pub type CookieJar = HashMap<String, String>;

#[derive(Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new(base_path: String) -> Configuration {
        Configuration {
            base_path,
            user_agent: Some("OpenAPI-Generator/1.9.0/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
            cookies: CookieJar::new(),
        }
    }
}
