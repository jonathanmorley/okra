#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationLicensing {
    #[serde(rename = "seatCount", skip_serializing_if = "Option::is_none")]
    r#seat_count: Option<i32>,
}

impl r#ApplicationLicensing {
    pub fn new(
    ) -> Self {
        Self {
          r#seat_count: None,
        }
    }

    pub fn set_seat_count(&mut self, r#seat_count: i32) {
        self.r#seat_count = Some(r#seat_count);
    }

    pub fn with_seat_count(mut self, r#seat_count: i32) -> Self {
        self.r#seat_count = Some(r#seat_count);
        self
    }

    pub fn r#seat_count(&self) -> Option<&i32> {
        self.r#seat_count.as_ref().map(|x| x.borrow())
    }

    pub fn reset_seat_count(&mut self) {
        self.r#seat_count = None;
    }
}
