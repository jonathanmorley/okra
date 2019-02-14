#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#PasswordPolicyPasswordSettingsComplexity {
    #[serde(rename = "dictionary", skip_serializing_if = "Option::is_none")]
    r#dictionary: Option<PasswordDictionary>,
    #[serde(rename = "excludeAttributes", skip_serializing_if = "Option::is_none")]
    r#exclude_attributes: Option<Vec<String>>,
    #[serde(rename = "excludeUsername", skip_serializing_if = "Option::is_none")]
    r#exclude_username: Option<bool>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    r#min_length: Option<i32>,
    #[serde(rename = "minLowerCase", skip_serializing_if = "Option::is_none")]
    r#min_lower_case: Option<i32>,
    #[serde(rename = "minNumber", skip_serializing_if = "Option::is_none")]
    r#min_number: Option<i32>,
    #[serde(rename = "minSymbol", skip_serializing_if = "Option::is_none")]
    r#min_symbol: Option<i32>,
    #[serde(rename = "minUpperCase", skip_serializing_if = "Option::is_none")]
    r#min_upper_case: Option<i32>,
}

impl r#PasswordPolicyPasswordSettingsComplexity {
    pub fn new(
    ) -> Self {
        Self {
          r#dictionary: None,
          r#exclude_attributes: None,
          r#exclude_username: None,
          r#min_length: None,
          r#min_lower_case: None,
          r#min_number: None,
          r#min_symbol: None,
          r#min_upper_case: None,
        }
    }

    pub fn set_dictionary(&mut self, r#dictionary: PasswordDictionary) {
        self.r#dictionary = Some(r#dictionary);
    }

    pub fn with_dictionary(mut self, r#dictionary: PasswordDictionary) -> Self {
        self.r#dictionary = Some(r#dictionary);
        self
    }

    pub fn r#dictionary(&self) -> Option<&PasswordDictionary> {
        self.r#dictionary.as_ref().map(|x| x.borrow())
    }

    pub fn reset_dictionary(&mut self) {
        self.r#dictionary = None;
    }

    pub fn set_exclude_attributes(&mut self, r#exclude_attributes: Vec<String>) {
        self.r#exclude_attributes = Some(r#exclude_attributes);
    }

    pub fn with_exclude_attributes(mut self, r#exclude_attributes: Vec<String>) -> Self {
        self.r#exclude_attributes = Some(r#exclude_attributes);
        self
    }

    pub fn r#exclude_attributes(&self) -> Option<&Vec<String>> {
        self.r#exclude_attributes.as_ref().map(|x| x.borrow())
    }

    pub fn reset_exclude_attributes(&mut self) {
        self.r#exclude_attributes = None;
    }

    pub fn set_exclude_username(&mut self, r#exclude_username: bool) {
        self.r#exclude_username = Some(r#exclude_username);
    }

    pub fn with_exclude_username(mut self, r#exclude_username: bool) -> Self {
        self.r#exclude_username = Some(r#exclude_username);
        self
    }

    pub fn r#exclude_username(&self) -> Option<&bool> {
        self.r#exclude_username.as_ref().map(|x| x.borrow())
    }

    pub fn reset_exclude_username(&mut self) {
        self.r#exclude_username = None;
    }

    pub fn set_min_length(&mut self, r#min_length: i32) {
        self.r#min_length = Some(r#min_length);
    }

    pub fn with_min_length(mut self, r#min_length: i32) -> Self {
        self.r#min_length = Some(r#min_length);
        self
    }

    pub fn r#min_length(&self) -> Option<&i32> {
        self.r#min_length.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_length(&mut self) {
        self.r#min_length = None;
    }

    pub fn set_min_lower_case(&mut self, r#min_lower_case: i32) {
        self.r#min_lower_case = Some(r#min_lower_case);
    }

    pub fn with_min_lower_case(mut self, r#min_lower_case: i32) -> Self {
        self.r#min_lower_case = Some(r#min_lower_case);
        self
    }

    pub fn r#min_lower_case(&self) -> Option<&i32> {
        self.r#min_lower_case.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_lower_case(&mut self) {
        self.r#min_lower_case = None;
    }

    pub fn set_min_number(&mut self, r#min_number: i32) {
        self.r#min_number = Some(r#min_number);
    }

    pub fn with_min_number(mut self, r#min_number: i32) -> Self {
        self.r#min_number = Some(r#min_number);
        self
    }

    pub fn r#min_number(&self) -> Option<&i32> {
        self.r#min_number.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_number(&mut self) {
        self.r#min_number = None;
    }

    pub fn set_min_symbol(&mut self, r#min_symbol: i32) {
        self.r#min_symbol = Some(r#min_symbol);
    }

    pub fn with_min_symbol(mut self, r#min_symbol: i32) -> Self {
        self.r#min_symbol = Some(r#min_symbol);
        self
    }

    pub fn r#min_symbol(&self) -> Option<&i32> {
        self.r#min_symbol.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_symbol(&mut self) {
        self.r#min_symbol = None;
    }

    pub fn set_min_upper_case(&mut self, r#min_upper_case: i32) {
        self.r#min_upper_case = Some(r#min_upper_case);
    }

    pub fn with_min_upper_case(mut self, r#min_upper_case: i32) -> Self {
        self.r#min_upper_case = Some(r#min_upper_case);
        self
    }

    pub fn r#min_upper_case(&self) -> Option<&i32> {
        self.r#min_upper_case.as_ref().map(|x| x.borrow())
    }

    pub fn reset_min_upper_case(&mut self) {
        self.r#min_upper_case = None;
    }
}
