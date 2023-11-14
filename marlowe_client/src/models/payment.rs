/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Payment : A Marlowe payment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "payment_from")]
    pub payment_from: Box<crate::models::Party>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::Payee>,
    #[serde(rename = "token")]
    pub token: Box<crate::models::Token>,
}

impl Payment {
    /// A Marlowe payment.
    pub fn new(amount: i32, payment_from: crate::models::Party, to: crate::models::Payee, token: crate::models::Token) -> Payment {
        Payment {
            amount,
            payment_from: Box::new(payment_from),
            to: Box::new(to),
            token: Box::new(token),
        }
    }
}

