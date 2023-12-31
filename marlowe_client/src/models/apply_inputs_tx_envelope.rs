/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ApplyInputsTxEnvelope : The \"type\" property of \"tx\" must be \"Tx BabbageEra\" or \"Tx ConwayEra\"

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyInputsTxEnvelope {
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "contractId")]
    pub contract_id: String,
    /// The hex-encoded identifier of a Cardano transaction
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "tx")]
    pub tx: Box<crate::models::TextEnvelope>,
}

impl ApplyInputsTxEnvelope {
    /// The \"type\" property of \"tx\" must be \"Tx BabbageEra\" or \"Tx ConwayEra\"
    pub fn new(
        contract_id: String,
        transaction_id: String,
        tx: crate::models::TextEnvelope,
    ) -> ApplyInputsTxEnvelope {
        ApplyInputsTxEnvelope {
            contract_id,
            transaction_id,
            tx: Box::new(tx),
        }
    }
}
