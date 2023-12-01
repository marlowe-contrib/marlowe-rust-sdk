#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;

// TODO: remove reqwest as pub when headers are actually handled by the openapi-generator-cli tool
pub use reqwest;
pub mod apis;
pub mod models;
