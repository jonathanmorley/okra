#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#ApplicationVisibility {
    #[serde(rename = "appLinks", skip_serializing_if = "Option::is_none")]
    r#app_links: Option<Value>,
    #[serde(rename = "autoSubmitToolbar", skip_serializing_if = "Option::is_none")]
    r#auto_submit_toolbar: Option<bool>,
    #[serde(rename = "hide", skip_serializing_if = "Option::is_none")]
    r#hide: Option<ApplicationVisibilityHide>,
}

impl r#ApplicationVisibility {
    pub fn new(
    ) -> Self {
        Self {
          r#app_links: None,
          r#auto_submit_toolbar: None,
          r#hide: None,
        }
    }

    pub fn set_app_links(&mut self, r#app_links: Value) {
        self.r#app_links = Some(r#app_links);
    }

    pub fn with_app_links(mut self, r#app_links: Value) -> Self {
        self.r#app_links = Some(r#app_links);
        self
    }

    pub fn r#app_links(&self) -> Option<&Value> {
        self.r#app_links.as_ref().map(|x| x.borrow())
    }

    pub fn reset_app_links(&mut self) {
        self.r#app_links = None;
    }

    pub fn set_auto_submit_toolbar(&mut self, r#auto_submit_toolbar: bool) {
        self.r#auto_submit_toolbar = Some(r#auto_submit_toolbar);
    }

    pub fn with_auto_submit_toolbar(mut self, r#auto_submit_toolbar: bool) -> Self {
        self.r#auto_submit_toolbar = Some(r#auto_submit_toolbar);
        self
    }

    pub fn r#auto_submit_toolbar(&self) -> Option<&bool> {
        self.r#auto_submit_toolbar.as_ref().map(|x| x.borrow())
    }

    pub fn reset_auto_submit_toolbar(&mut self) {
        self.r#auto_submit_toolbar = None;
    }

    pub fn set_hide(&mut self, r#hide: ApplicationVisibilityHide) {
        self.r#hide = Some(r#hide);
    }

    pub fn with_hide(mut self, r#hide: ApplicationVisibilityHide) -> Self {
        self.r#hide = Some(r#hide);
        self
    }

    pub fn r#hide(&self) -> Option<&ApplicationVisibilityHide> {
        self.r#hide.as_ref().map(|x| x.borrow())
    }

    pub fn reset_hide(&mut self) {
        self.r#hide = None;
    }
}
