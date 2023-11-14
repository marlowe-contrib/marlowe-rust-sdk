/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionError : A Marlowe transaction error.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionError {
    #[serde(rename = "context")]
    pub context: Box<crate::models::IntervalError>,
    #[serde(rename = "error")]
    pub error: Error,
}

impl TransactionError {
    /// A Marlowe transaction error.
    pub fn new(context: crate::models::IntervalError, error: Error) -> TransactionError {
        TransactionError {
            context: Box::new(context),
            error,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    #[serde(rename = "TEIntervalError")]
    TeIntervalError,
}

impl Default for Error {
    fn default() -> Error {
        Self::TeIntervalError
    }
}

