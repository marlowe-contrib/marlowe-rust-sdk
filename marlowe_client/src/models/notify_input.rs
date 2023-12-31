/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// NotifyInput : Notify a contract to check a condition

/// Notify a contract to check a condition
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotifyInput {
    #[serde(rename = "input_notify")]
    InputNotify,
}

impl ToString for NotifyInput {
    fn to_string(&self) -> String {
        match self {
            Self::InputNotify => String::from("input_notify"),
        }
    }
}

impl Default for NotifyInput {
    fn default() -> NotifyInput {
        Self::InputNotify
    }
}
