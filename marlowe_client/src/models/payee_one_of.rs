/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayeeOneOf : Pays funds into a party's account in the contract.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayeeOneOf {
    #[serde(rename = "account")]
    pub account: Box<crate::models::Party>,
}

impl PayeeOneOf {
    /// Pays funds into a party's account in the contract.
    pub fn new(account: crate::models::Party) -> PayeeOneOf {
        PayeeOneOf {
            account: Box::new(account),
        }
    }
}


