/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContractObjectOneOf1 : If an observation is true, the first contract applies, otherwise the second contract applies.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractObjectOneOf1 {
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::ContractObject>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::ObservationObject>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
}

impl ContractObjectOneOf1 {
    /// If an observation is true, the first contract applies, otherwise the second contract applies.
    pub fn new(r#else: crate::models::ContractObject, r#if: crate::models::ObservationObject, then: crate::models::ContractObject) -> ContractObjectOneOf1 {
        ContractObjectOneOf1 {
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            then: Box::new(then),
        }
    }
}


