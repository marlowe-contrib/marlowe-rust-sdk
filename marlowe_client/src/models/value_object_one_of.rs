/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueObjectOneOf {
    #[serde(rename = "amount_of_token")]
    pub amount_of_token: Box<crate::models::TokenObject>,
    #[serde(rename = "in_account")]
    pub in_account: Box<crate::models::PartyObject>,
}

impl ValueObjectOneOf {
    pub fn new(amount_of_token: crate::models::TokenObject, in_account: crate::models::PartyObject) -> ValueObjectOneOf {
        ValueObjectOneOf {
            amount_of_token: Box::new(amount_of_token),
            in_account: Box::new(in_account),
        }
    }
}

