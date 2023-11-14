/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InputOneOf3 : Deposit funds into an account in a contract and provide the continuation of the contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputOneOf3 {
    #[serde(rename = "continuation_hash")]
    pub continuation_hash: String,
    #[serde(rename = "input_from_party")]
    pub input_from_party: Box<crate::models::Party>,
    #[serde(rename = "into_account")]
    pub into_account: Box<crate::models::Party>,
    #[serde(rename = "merkleized_continuation")]
    pub merkleized_continuation: Box<crate::models::Contract>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "that_deposits")]
    pub that_deposits: i32,
}

impl InputOneOf3 {
    /// Deposit funds into an account in a contract and provide the continuation of the contract
    pub fn new(continuation_hash: String, input_from_party: crate::models::Party, into_account: crate::models::Party, merkleized_continuation: crate::models::Contract, of_token: crate::models::Token, that_deposits: i32) -> InputOneOf3 {
        InputOneOf3 {
            continuation_hash,
            input_from_party: Box::new(input_from_party),
            into_account: Box::new(into_account),
            merkleized_continuation: Box::new(merkleized_continuation),
            of_token: Box::new(of_token),
            that_deposits,
        }
    }
}

