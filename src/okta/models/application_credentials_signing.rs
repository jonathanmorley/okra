#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationCredentialsSigning {
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    r#kid: Option<String>,
    #[serde(rename = "lastRotated", skip_serializing_if = "Option::is_none")]
    r#last_rotated: Option<String>,
    #[serde(rename = "nextRotation", skip_serializing_if = "Option::is_none")]
    r#next_rotation: Option<String>,
    #[serde(rename = "rotationMode", skip_serializing_if = "Option::is_none")]
    r#rotation_mode: Option<String>,
}

impl r#ApplicationCredentialsSigning {
    pub fn new(
    ) -> Self {
        Self {
          r#kid: None,
          r#last_rotated: None,
          r#next_rotation: None,
          r#rotation_mode: None,
        }
    }

    pub fn set_kid(&mut self, r#kid: String) {
        self.r#kid = Some(r#kid);
    }

    pub fn with_kid(mut self, r#kid: String) -> Self {
        self.r#kid = Some(r#kid);
        self
    }

    pub fn r#kid(&self) -> Option<&str> {
        self.r#kid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_kid(&mut self) {
        self.r#kid = None;
    }

    pub fn set_last_rotated(&mut self, r#last_rotated: String) {
        self.r#last_rotated = Some(r#last_rotated);
    }

    pub fn with_last_rotated(mut self, r#last_rotated: String) -> Self {
        self.r#last_rotated = Some(r#last_rotated);
        self
    }

    pub fn r#last_rotated(&self) -> Option<&str> {
        self.r#last_rotated.as_ref().map(|x| x.borrow())
    }

    pub fn reset_last_rotated(&mut self) {
        self.r#last_rotated = None;
    }

    pub fn set_next_rotation(&mut self, r#next_rotation: String) {
        self.r#next_rotation = Some(r#next_rotation);
    }

    pub fn with_next_rotation(mut self, r#next_rotation: String) -> Self {
        self.r#next_rotation = Some(r#next_rotation);
        self
    }

    pub fn r#next_rotation(&self) -> Option<&str> {
        self.r#next_rotation.as_ref().map(|x| x.borrow())
    }

    pub fn reset_next_rotation(&mut self) {
        self.r#next_rotation = None;
    }

    pub fn set_rotation_mode(&mut self, r#rotation_mode: String) {
        self.r#rotation_mode = Some(r#rotation_mode);
    }

    pub fn with_rotation_mode(mut self, r#rotation_mode: String) -> Self {
        self.r#rotation_mode = Some(r#rotation_mode);
        self
    }

    pub fn r#rotation_mode(&self) -> Option<&str> {
        self.r#rotation_mode.as_ref().map(|x| x.borrow())
    }

    pub fn reset_rotation_mode(&mut self) {
        self.r#rotation_mode = None;
    }
}
