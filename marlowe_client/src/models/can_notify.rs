/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CanNotify : Notify Input tha can be applied for a given contract

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanNotify {
    /// Index of a \"Case Action\" in a \"When\"
    #[serde(rename = "case_index")]
    pub case_index: i32,
    /// Indicates if a given contract continuation is merkleized
    #[serde(rename = "is_merkleized_continuation")]
    pub is_merkleized_continuation: bool,
}

impl CanNotify {
    /// Notify Input tha can be applied for a given contract
    pub fn new(case_index: i32, is_merkleized_continuation: bool) -> CanNotify {
        CanNotify {
            case_index,
            is_merkleized_continuation,
        }
    }
}
