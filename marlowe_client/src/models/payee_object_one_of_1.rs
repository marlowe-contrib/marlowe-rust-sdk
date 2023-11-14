/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayeeObjectOneOf1 : Pays funds to a party.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayeeObjectOneOf1 {
    #[serde(rename = "party")]
    pub party: Box<crate::models::PartyObject>,
}

impl PayeeObjectOneOf1 {
    /// Pays funds to a party.
    pub fn new(party: crate::models::PartyObject) -> PayeeObjectOneOf1 {
        PayeeObjectOneOf1 {
            party: Box::new(party),
        }
    }
}

