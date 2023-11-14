/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InputOneOf : Notify a contract to check a condition and provide the continuation of the contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputOneOf {
    #[serde(rename = "continuation_hash")]
    pub continuation_hash: String,
    #[serde(rename = "merkleized_continuation")]
    pub merkleized_continuation: Box<crate::models::Contract>,
}

impl InputOneOf {
    /// Notify a contract to check a condition and provide the continuation of the contract
    pub fn new(continuation_hash: String, merkleized_continuation: crate::models::Contract) -> InputOneOf {
        InputOneOf {
            continuation_hash,
            merkleized_continuation: Box::new(merkleized_continuation),
        }
    }
}

