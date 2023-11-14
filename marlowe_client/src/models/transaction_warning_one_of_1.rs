/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionWarningOneOf1 : A warning for a non-positive payment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionWarningOneOf1 {
    #[serde(rename = "account")]
    pub account: Box<crate::models::Party>,
    #[serde(rename = "asked_to_pay")]
    pub asked_to_pay: i32,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "to_payee")]
    pub to_payee: Box<crate::models::Payee>,
}

impl TransactionWarningOneOf1 {
    /// A warning for a non-positive payment.
    pub fn new(account: crate::models::Party, asked_to_pay: i32, of_token: crate::models::Token, to_payee: crate::models::Payee) -> TransactionWarningOneOf1 {
        TransactionWarningOneOf1 {
            account: Box::new(account),
            asked_to_pay,
            of_token: Box::new(of_token),
            to_payee: Box::new(to_payee),
        }
    }
}


