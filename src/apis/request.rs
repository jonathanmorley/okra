use std::collections::HashMap;

use super::{configuration, Error};
use futures;
use futures::Future;
use hyper;
use serde;
use serde_json;

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

    pub fn with_body_param<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_header_param(mut self, basename: String, param: String) -> Self {
        self.header_params.insert(basename, param);
        self
    }

    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    pub fn with_form_param(mut self, basename: String, param: String) -> Self {
        self.form_params.insert(basename, param);
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn execute<'a, U>(
        self,
        conf: &configuration::Configuration,
    ) -> Box<Future<Item = U, Error = Error<serde_json::Value>> + 'a>
    where
        U: Sized + 'a,
        for<'de> U: serde::Deserialize<'de>,
    {
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

        let res = req.send().map_err(|e| Error::from(e)).and_then(|mut resp| {
            let status = resp.status();
            if status.is_success() {
                if self.no_return_type {
                    serde_json::from_str("null").map_err(|e| e.into())
                } else {
                    resp.json().map_err(|e| e.into())
                }
            } else {
                match resp.text() {
                    Ok(body) => Err(Error::from((status, body.as_bytes()))),
                    Err(e) => Err(e.into()),
                }
            }
        });

        Box::new(futures::future::result(res))
    }
}
