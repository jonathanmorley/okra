#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogGeographicalContext {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    r#city: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    r#country: Option<String>,
    #[serde(rename = "geolocation", skip_serializing_if = "Option::is_none")]
    r#geolocation: Option<LogGeolocation>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    r#postal_code: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    r#state: Option<String>,
}

impl r#LogGeographicalContext {
    pub fn new(
    ) -> Self {
        Self {
          r#city: None,
          r#country: None,
          r#geolocation: None,
          r#postal_code: None,
          r#state: None,
        }
    }

    pub fn set_city(&mut self, r#city: String) {
        self.r#city = Some(r#city);
    }

    pub fn with_city(mut self, r#city: String) -> Self {
        self.r#city = Some(r#city);
        self
    }

    pub fn r#city(&self) -> Option<&str> {
        self.r#city.as_ref().map(|x| x.borrow())
    }

    pub fn reset_city(&mut self) {
        self.r#city = None;
    }

    pub fn set_country(&mut self, r#country: String) {
        self.r#country = Some(r#country);
    }

    pub fn with_country(mut self, r#country: String) -> Self {
        self.r#country = Some(r#country);
        self
    }

    pub fn r#country(&self) -> Option<&str> {
        self.r#country.as_ref().map(|x| x.borrow())
    }

    pub fn reset_country(&mut self) {
        self.r#country = None;
    }

    pub fn set_geolocation(&mut self, r#geolocation: LogGeolocation) {
        self.r#geolocation = Some(r#geolocation);
    }

    pub fn with_geolocation(mut self, r#geolocation: LogGeolocation) -> Self {
        self.r#geolocation = Some(r#geolocation);
        self
    }

    pub fn r#geolocation(&self) -> Option<&LogGeolocation> {
        self.r#geolocation.as_ref().map(|x| x.borrow())
    }

    pub fn reset_geolocation(&mut self) {
        self.r#geolocation = None;
    }

    pub fn set_postal_code(&mut self, r#postal_code: String) {
        self.r#postal_code = Some(r#postal_code);
    }

    pub fn with_postal_code(mut self, r#postal_code: String) -> Self {
        self.r#postal_code = Some(r#postal_code);
        self
    }

    pub fn r#postal_code(&self) -> Option<&str> {
        self.r#postal_code.as_ref().map(|x| x.borrow())
    }

    pub fn reset_postal_code(&mut self) {
        self.r#postal_code = None;
    }

    pub fn set_state(&mut self, r#state: String) {
        self.r#state = Some(r#state);
    }

    pub fn with_state(mut self, r#state: String) -> Self {
        self.r#state = Some(r#state);
        self
    }

    pub fn r#state(&self) -> Option<&str> {
        self.r#state.as_ref().map(|x| x.borrow())
    }

    pub fn reset_state(&mut self) {
        self.r#state = None;
    }
}
