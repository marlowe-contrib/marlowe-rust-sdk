/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayeeOneOf1 : Pays funds to a party.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayeeOneOf1 {
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
}

impl PayeeOneOf1 {
    /// Pays funds to a party.
    pub fn new(party: crate::models::Party) -> PayeeOneOf1 {
        PayeeOneOf1 {
            party: Box::new(party),
        }
    }
}

