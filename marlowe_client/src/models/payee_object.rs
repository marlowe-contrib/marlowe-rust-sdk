/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayeeObject : A recipient of a payment



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayeeObject {
    #[serde(rename = "account")]
    pub account: Box<crate::models::PartyObject>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::PartyObject>,
}

impl PayeeObject {
    /// A recipient of a payment
    pub fn new(account: crate::models::PartyObject, party: crate::models::PartyObject) -> PayeeObject {
        PayeeObject {
            account: Box::new(account),
            party: Box::new(party),
        }
    }
}


