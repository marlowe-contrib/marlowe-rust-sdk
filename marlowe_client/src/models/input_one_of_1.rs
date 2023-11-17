/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InputOneOf1 : Make a choice in a contract and provide the continuation of the contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputOneOf1 {
    #[serde(rename = "continuation_hash")]
    pub continuation_hash: String,
    #[serde(rename = "for_choice_id")]
    pub for_choice_id: Box<crate::models::ChoiceId>,
    #[serde(rename = "input_that_chooses_num")]
    pub input_that_chooses_num: i32,
    #[serde(rename = "merkleized_continuation")]
    pub merkleized_continuation: Box<crate::models::Contract>,
}

impl InputOneOf1 {
    /// Make a choice in a contract and provide the continuation of the contract
    pub fn new(continuation_hash: String, for_choice_id: crate::models::ChoiceId, input_that_chooses_num: i32, merkleized_continuation: crate::models::Contract) -> InputOneOf1 {
        InputOneOf1 {
            continuation_hash,
            for_choice_id: Box::new(for_choice_id),
            input_that_chooses_num,
            merkleized_continuation: Box::new(merkleized_continuation),
        }
    }
}


