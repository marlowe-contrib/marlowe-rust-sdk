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
pub struct TokenInAccountObject {
    #[serde(rename = "amount_of_token")]
    pub amount_of_token: Box<crate::models::TokenObject>,
    #[serde(rename = "in_account")]
    pub in_account: Box<crate::models::PartyObject>,
}

impl TokenInAccountObject {
    pub fn new(
        amount_of_token: crate::models::TokenObject,
        in_account: crate::models::PartyObject,
    ) -> TokenInAccountObject {
        TokenInAccountObject {
            amount_of_token: Box::new(amount_of_token),
            in_account: Box::new(in_account),
        }
    }
}