/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionOutput : Marlowe transaction output.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionOutput {
    #[serde(rename = "contract")]
    pub contract: Box<crate::models::Contract>,
    #[serde(rename = "payments")]
    pub payments: Vec<crate::models::Payment>,
    #[serde(rename = "state")]
    pub state: Box<crate::models::MarloweState>,
    #[serde(rename = "warnings")]
    pub warnings: Vec<crate::models::TransactionWarning>,
    #[serde(rename = "transaction_error")]
    pub transaction_error: Box<crate::models::TransactionError>,
}

impl TransactionOutput {
    /// Marlowe transaction output.
    pub fn new(
        contract: crate::models::Contract,
        payments: Vec<crate::models::Payment>,
        state: crate::models::MarloweState,
        warnings: Vec<crate::models::TransactionWarning>,
        transaction_error: crate::models::TransactionError,
    ) -> TransactionOutput {
        TransactionOutput {
            contract: Box::new(contract),
            payments,
            state: Box::new(state),
            warnings,
            transaction_error: Box::new(transaction_error),
        }
    }
}
