/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartialPayment : A warning for partial payment.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialPayment {
    #[serde(rename = "account")]
    pub account: Box<crate::models::Party>,
    #[serde(rename = "asked_to_pay")]
    pub asked_to_pay: i32,
    #[serde(rename = "but_only_paid")]
    pub but_only_paid: i32,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "to_payee")]
    pub to_payee: Box<crate::models::Payee>,
}

impl PartialPayment {
    /// A warning for partial payment.
    pub fn new(
        account: crate::models::Party,
        asked_to_pay: i32,
        but_only_paid: i32,
        of_token: crate::models::Token,
        to_payee: crate::models::Payee,
    ) -> PartialPayment {
        PartialPayment {
            account: Box::new(account),
            asked_to_pay,
            but_only_paid,
            of_token: Box::new(of_token),
            to_payee: Box::new(to_payee),
        }
    }
}
