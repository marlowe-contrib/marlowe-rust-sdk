/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartyObject : A participant in a contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartyObject {
    #[serde(rename = "role_token")]
    pub role_token: String,
    /// A cardano address
    #[serde(rename = "address")]
    pub address: String,
    /// An arbitrary text identifier for an object in a Marlowe object bundle.
    #[serde(rename = "ref")]
    pub r#ref: String,
}

impl PartyObject {
    /// A participant in a contract
    pub fn new(role_token: String, address: String, r#ref: String) -> PartyObject {
        PartyObject {
            role_token,
            address,
            r#ref,
        }
    }
}


