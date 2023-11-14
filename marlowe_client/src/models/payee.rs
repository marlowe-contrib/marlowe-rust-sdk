/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Payee : A recipient of a payment



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payee {
    #[serde(rename = "account")]
    pub account: Box<crate::models::Party>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
}

impl Payee {
    /// A recipient of a payment
    pub fn new(account: crate::models::Party, party: crate::models::Party) -> Payee {
        Payee {
            account: Box::new(account),
            party: Box::new(party),
        }
    }
}


