use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod application_api;
pub use self::application_api::{ ApplicationApi, ApplicationApiClient };
mod authentication_api;
pub use self::authentication_api::{ AuthenticationApi, AuthenticationApiClient };
mod group_api;
pub use self::group_api::{ GroupApi, GroupApiClient };
mod log_api;
pub use self::log_api::{ LogApi, LogApiClient };
mod policy_api;
pub use self::policy_api::{ PolicyApi, PolicyApiClient };
mod session_api;
pub use self::session_api::{ SessionApi, SessionApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };
mod user_factor_api;
pub use self::user_factor_api::{ UserFactorApi, UserFactorApiClient };

pub mod configuration;
pub mod client;
