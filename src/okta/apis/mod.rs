use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
where
T: serde::Deserialize<'de>,
{
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e);
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

pub mod request;
pub mod configuration;
mod authentication_api;
pub use self::authentication_api::r#AuthenticationApiClient;
mod log_api;
pub use self::log_api::r#LogApiClient;
mod session_api;
pub use self::session_api::r#SessionApiClient;
mod application_api;
pub use self::application_api::r#ApplicationApiClient;
mod group_api;
pub use self::group_api::r#GroupApiClient;
mod policy_api;
pub use self::policy_api::r#PolicyApiClient;
mod user_factor_api;
pub use self::user_factor_api::r#UserFactorApiClient;
mod user_api;
pub use self::user_api::r#UserApiClient;
