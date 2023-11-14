/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionOutputOneOf1 : Marlowe transaction error.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionOutputOneOf1 {
    #[serde(rename = "transaction_error")]
    pub transaction_error: Box<crate::models::TransactionError>,
}

impl TransactionOutputOneOf1 {
    /// Marlowe transaction error.
    pub fn new(transaction_error: crate::models::TransactionError) -> TransactionOutputOneOf1 {
        TransactionOutputOneOf1 {
            transaction_error: Box::new(transaction_error),
        }
    }
}


