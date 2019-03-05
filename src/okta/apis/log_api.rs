use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct LogApiClient {
    configuration: Configuration,
}

impl LogApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#get_logs(
        &self,
        r#until: String,
        r#since: String,
        r#filter: String,
        r#q: String,
        r#limit: i32,
        r#sort_order: String,
        r#after: String,
    ) -> Result<Vec<LogEvent>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/api/v1/logs".to_string(),
        )
        .with_query_param("until".to_string(), r#until.to_string())
        .with_query_param("since".to_string(), r#since.to_string())
        .with_query_param("filter".to_string(), r#filter.to_string())
        .with_query_param("q".to_string(), r#q.to_string())
        .with_query_param("limit".to_string(), r#limit.to_string())
        .with_query_param("sortOrder".to_string(), r#sort_order.to_string())
        .with_query_param("after".to_string(), r#after.to_string())
        .execute(self.configuration.borrow())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use tc_core::{Container, Image};
    use tc_generic::{GenericImage, WaitFor};
    use testcontainers::*;
    #[test]
    fn r#get_logs() {
        client().r#get_logs(
          "until".into(),
          "since".into(),
          "filter".into(),
          "q".into(),
          i32::default(),
          "sortOrder".into(),
          "after".into(),
        ).unwrap();
    }

    

    fn client() -> super::LogApiClient {
        std::process::Command::new("docker")
                  .args(&["build", "-t=test-apisprout", "."])
                  .output()
                  .expect("failed to execute process");

        let testcontainer_docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = testcontainer_docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::LogApiClient::new(configuration)
    }
}

