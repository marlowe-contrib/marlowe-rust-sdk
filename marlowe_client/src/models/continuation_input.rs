/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContinuationInput : Notify a contract to check a condition and provide the continuation of the contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuationInput {
    #[serde(rename = "continuation_hash")]
    pub continuation_hash: String,
    #[serde(rename = "merkleized_continuation")]
    pub merkleized_continuation: Box<crate::models::Contract>,
}

impl ContinuationInput {
    /// Notify a contract to check a condition and provide the continuation of the contract
    pub fn new(continuation_hash: String, merkleized_continuation: crate::models::Contract) -> ContinuationInput {
        ContinuationInput {
            continuation_hash,
            merkleized_continuation: Box::new(merkleized_continuation),
        }
    }
}

