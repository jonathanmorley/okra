#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#LogGeolocation {
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    r#lat: Option<f64>,
    #[serde(rename = "lon", skip_serializing_if = "Option::is_none")]
    r#lon: Option<f64>,
}

impl r#LogGeolocation {
    pub fn new(
    ) -> Self {
        Self {
          r#lat: None,
          r#lon: None,
        }
    }

    pub fn set_lat(&mut self, r#lat: f64) {
        self.r#lat = Some(r#lat);
    }

    pub fn with_lat(mut self, r#lat: f64) -> Self {
        self.r#lat = Some(r#lat);
        self
    }

    pub fn r#lat(&self) -> Option<&f64> {
        self.r#lat.as_ref().map(|x| x.borrow())
    }

    pub fn reset_lat(&mut self) {
        self.r#lat = None;
    }

    pub fn set_lon(&mut self, r#lon: f64) {
        self.r#lon = Some(r#lon);
    }

    pub fn with_lon(mut self, r#lon: f64) -> Self {
        self.r#lon = Some(r#lon);
        self
    }

    pub fn r#lon(&self) -> Option<&f64> {
        self.r#lon.as_ref().map(|x| x.borrow())
    }

    pub fn reset_lon(&mut self) {
        self.r#lon = None;
    }
}
