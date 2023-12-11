/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepositInput : Deposit funds into an account in a contract



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositInput {
    #[serde(rename = "input_from_party")]
    pub input_from_party: Box<crate::models::Party>,
    #[serde(rename = "into_account")]
    pub into_account: Box<crate::models::Party>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "that_deposits")]
    pub that_deposits: i32,
}

impl DepositInput {
    /// Deposit funds into an account in a contract
    pub fn new(input_from_party: crate::models::Party, into_account: crate::models::Party, of_token: crate::models::Token, that_deposits: i32) -> DepositInput {
        DepositInput {
            input_from_party: Box::new(input_from_party),
            into_account: Box::new(into_account),
            of_token: Box::new(of_token),
            that_deposits,
        }
    }
}


