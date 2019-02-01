#[macro_use]
extern crate serde_derive;

pub mod apis;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
