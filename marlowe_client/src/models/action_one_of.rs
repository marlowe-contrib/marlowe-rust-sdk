/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionOneOf {
    #[serde(rename = "deposits")]
    pub deposits: Box<crate::models::Value>,
    #[serde(rename = "into_account")]
    pub into_account: Box<crate::models::Party>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
}

impl ActionOneOf {
    pub fn new(deposits: crate::models::Value, into_account: crate::models::Party, of_token: crate::models::Token, party: crate::models::Party) -> ActionOneOf {
        ActionOneOf {
            deposits: Box::new(deposits),
            into_account: Box::new(into_account),
            of_token: Box::new(of_token),
            party: Box::new(party),
        }
    }
}


