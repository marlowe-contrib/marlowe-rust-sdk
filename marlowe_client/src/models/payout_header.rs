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
pub struct PayoutHeader {
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "contractId")]
    pub contract_id: String,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "payoutId")]
    pub payout_id: String,
    #[serde(rename = "role")]
    pub role: Box<crate::models::AssetId>,
    #[serde(rename = "status")]
    pub status: crate::models::PayoutStatus,
    /// The hex-encoded identifier of a Cardano transaction
    #[serde(rename = "withdrawalId", skip_serializing_if = "Option::is_none")]
    pub withdrawal_id: Option<String>,
}

impl PayoutHeader {
    pub fn new(contract_id: String, payout_id: String, role: crate::models::AssetId, status: crate::models::PayoutStatus) -> PayoutHeader {
        PayoutHeader {
            contract_id,
            payout_id,
            role: Box::new(role),
            status,
            withdrawal_id: None,
        }
    }
}


