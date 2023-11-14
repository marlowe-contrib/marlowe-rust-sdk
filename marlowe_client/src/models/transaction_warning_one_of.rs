/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionWarningOneOf : A warning for a non-positive deposit.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionWarningOneOf {
    #[serde(rename = "asked_to_deposit")]
    pub asked_to_deposit: i32,
    #[serde(rename = "in_account")]
    pub in_account: Box<crate::models::Party>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
}

impl TransactionWarningOneOf {
    /// A warning for a non-positive deposit.
    pub fn new(asked_to_deposit: i32, in_account: crate::models::Party, of_token: crate::models::Token, party: crate::models::Party) -> TransactionWarningOneOf {
        TransactionWarningOneOf {
            asked_to_deposit,
            in_account: Box::new(in_account),
            of_token: Box::new(of_token),
            party: Box::new(party),
        }
    }
}


