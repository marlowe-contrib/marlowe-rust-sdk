/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InputOneOf2 : Make a choice in a contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputOneOf2 {
    #[serde(rename = "for_choice_id")]
    pub for_choice_id: Box<crate::models::ChoiceId>,
    #[serde(rename = "input_that_chooses_num")]
    pub input_that_chooses_num: i32,
}

impl InputOneOf2 {
    /// Make a choice in a contract
    pub fn new(for_choice_id: crate::models::ChoiceId, input_that_chooses_num: i32) -> InputOneOf2 {
        InputOneOf2 {
            for_choice_id: Box::new(for_choice_id),
            input_that_chooses_num,
        }
    }
}

