use super::configuration;

use hyper;
use serde;
use serde_json;
use std::collections::HashMap;

pub(crate) struct ApiKey {
    pub in_header: bool,
    pub in_query: bool,
    pub param_name: String,
}

impl ApiKey {
    fn key(&self, prefix: &Option<String>, key: &str) -> String {
        match prefix {
            None => key.to_owned(),
            Some(ref prefix) => format!("{} {}", prefix, key),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum Auth {
    None,
    ApiKey(ApiKey),
    Basic,
    Oauth,
}

pub(crate) struct Request {
    auth: Auth,
    method: hyper::Method,
    path: String,
    query_params: HashMap<String, String>,
    no_return_type: bool,
    path_params: HashMap<String, String>,
    form_params: HashMap<String, String>,
    header_params: HashMap<String, String>,
    // TODO: multiple body params are possible technically, but not supported here.
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: hyper::Method, path: String) -> Self {
        Request {
            auth: Auth::None,
            method: method,
            path: path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            form_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    #[allow(dead_code)]
    pub fn with_header_param(mut self, basename: String, param: String) -> Self {
        self.header_params.insert(basename, param);
        self
    }

    #[allow(dead_code)]
    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    #[allow(dead_code)]
    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    #[allow(dead_code)]
    pub fn with_form_param(mut self, basename: String, param: String) -> Self {
        self.form_params.insert(basename, param);
        self
    }

    #[allow(dead_code)]
    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    #[allow(dead_code)]
    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn response(
        self,
        conf: &configuration::Configuration,
    ) -> Result<reqwest::Response, failure::Error> {
        let mut path = self.path.clone();
        for (k, v) in self.path_params.iter() {
            // replace {id} with the value of the id path param
            path = path.replace(&format!("{{{}}}", k), v);
        }

        let uri_str = format!("{}{}", conf.base_path, path);

        let mut req = conf.client.request(self.method.clone(), &uri_str);

        for (k, v) in self.header_params.iter() {
            req = req.header(k.as_str(), v.as_str());
        }

        let cookies = conf
            .cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(";");

        req = req.header("Cookie", cookies);

        for pair in self.query_params.iter() {
            req = req.query(&[pair]);
        }

        match self.auth {
            Auth::ApiKey(ref apikey) => {
                if let Some(ref key) = conf.api_key {
                    let val = apikey.key(&key.prefix, &key.key);
                    if apikey.in_query {
                        req = req.query(&[(&apikey.param_name, &val)]);
                    }
                    if apikey.in_header {
                        req = req.header(apikey.param_name.as_str(), val.as_str());
                    }
                }
            }
            Auth::Basic => {
                if let Some(ref auth_conf) = conf.basic_auth {
                    req = req.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
                }
            }
            Auth::Oauth => {
                if let Some(ref token) = conf.oauth_access_token {
                    req = req.bearer_auth(token.to_owned());
                }
            }
            Auth::None => {}
        }

        if let Some(user_agent) = conf.user_agent.as_ref() {
            req = req.header("User-Agent", user_agent.as_str());
        }

        if self.form_params.len() > 0 {
            req = req.form(&self.form_params);
        }

        if let Some(body) = self.serialized_body.clone() {
            req = req.header("Content-Type", "application/json").body(body);
        }

        req.send()?.error_for_status().map_err(|e| e.into())
    }

    pub fn execute<'a, U>(self, conf: &configuration::Configuration) -> Result<U, failure::Error>
    where
        U: Sized + 'a,
        for<'de> U: serde::Deserialize<'de>,
    {
        if self.no_return_type {
            serde_json::from_str("null").map_err(|e| e.into())
        } else {
            self.response(conf)?.json().map_err(|e| e.into())
        }
    }
}
