/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContractObjectOneOf3 : Bind a value to a name within the scope of a sub-contract.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractObjectOneOf3 {
    #[serde(rename = "be")]
    pub be: Box<crate::models::ValueObject>,
    #[serde(rename = "let")]
    pub r#let: String,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
}

impl ContractObjectOneOf3 {
    /// Bind a value to a name within the scope of a sub-contract.
    pub fn new(be: crate::models::ValueObject, r#let: String, then: crate::models::ContractObject) -> ContractObjectOneOf3 {
        ContractObjectOneOf3 {
            be: Box::new(be),
            r#let,
            then: Box::new(then),
        }
    }
}


