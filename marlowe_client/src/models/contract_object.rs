/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContractObject : Contract terms specified in Marlowe



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractObject {
    #[serde(rename = "from_account")]
    pub from_account: Box<crate::models::PartyObject>,
    #[serde(rename = "pay")]
    pub pay: Box<crate::models::ValueObject>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::PayeeObject>,
    #[serde(rename = "token")]
    pub token: Box<crate::models::TokenObject>,
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::ContractObject>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::ObservationObject>,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "timeout_continuation")]
    pub timeout_continuation: Box<crate::models::ContractObject>,
    #[serde(rename = "when")]
    pub when: Vec<crate::models::CaseObject>,
    #[serde(rename = "be")]
    pub be: Box<crate::models::ValueObject>,
    #[serde(rename = "let")]
    pub r#let: String,
    #[serde(rename = "assert")]
    pub assert: Box<crate::models::ObservationObject>,
    /// An arbitrary text identifier for an object in a Marlowe object bundle.
    #[serde(rename = "ref")]
    pub r#ref: String,
}

impl ContractObject {
    /// Contract terms specified in Marlowe
    pub fn new(from_account: crate::models::PartyObject, pay: crate::models::ValueObject, then: crate::models::ContractObject, to: crate::models::PayeeObject, token: crate::models::TokenObject, r#else: crate::models::ContractObject, r#if: crate::models::ObservationObject, timeout: i32, timeout_continuation: crate::models::ContractObject, when: Vec<crate::models::CaseObject>, be: crate::models::ValueObject, r#let: String, assert: crate::models::ObservationObject, r#ref: String) -> ContractObject {
        ContractObject {
            from_account: Box::new(from_account),
            pay: Box::new(pay),
            then: Box::new(then),
            to: Box::new(to),
            token: Box::new(token),
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            timeout,
            timeout_continuation: Box::new(timeout_continuation),
            when,
            be: Box::new(be),
            r#let,
            assert: Box::new(assert),
            r#ref,
        }
    }
}


