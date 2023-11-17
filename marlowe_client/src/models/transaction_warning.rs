/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionWarning : A transaction semantics warning.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionWarning {
    #[serde(rename = "asked_to_deposit")]
    pub asked_to_deposit: i32,
    #[serde(rename = "in_account")]
    pub in_account: Box<crate::models::Party>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
    #[serde(rename = "account")]
    pub account: Box<crate::models::Party>,
    #[serde(rename = "asked_to_pay")]
    pub asked_to_pay: i32,
    #[serde(rename = "to_payee")]
    pub to_payee: Box<crate::models::Payee>,
    #[serde(rename = "but_only_paid")]
    pub but_only_paid: i32,
    #[serde(rename = "had_value")]
    pub had_value: i32,
    #[serde(rename = "is_now_assigned")]
    pub is_now_assigned: i32,
    #[serde(rename = "value_id")]
    pub value_id: String,
}

impl TransactionWarning {
    /// A transaction semantics warning.
    pub fn new(asked_to_deposit: i32, in_account: crate::models::Party, of_token: crate::models::Token, party: crate::models::Party, account: crate::models::Party, asked_to_pay: i32, to_payee: crate::models::Payee, but_only_paid: i32, had_value: i32, is_now_assigned: i32, value_id: String) -> TransactionWarning {
        TransactionWarning {
            asked_to_deposit,
            in_account: Box::new(in_account),
            of_token: Box::new(of_token),
            party: Box::new(party),
            account: Box::new(account),
            asked_to_pay,
            to_payee: Box::new(to_payee),
            but_only_paid,
            had_value,
            is_now_assigned,
            value_id,
        }
    }
}


