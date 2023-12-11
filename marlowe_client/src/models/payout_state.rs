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
pub struct PayoutState {
    #[serde(rename = "assets")]
    pub assets: Box<crate::models::Assets>,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "contractId")]
    pub contract_id: String,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "payoutId")]
    pub payout_id: String,
    /// A cardano address, in Bech32 format
    #[serde(rename = "payoutValidatorAddress")]
    pub payout_validator_address: String,
    #[serde(rename = "role")]
    pub role: Box<crate::models::AssetId>,
    #[serde(rename = "status")]
    pub status: crate::models::PayoutStatus,
    /// The hex-encoded identifier of a Cardano transaction
    #[serde(rename = "withdrawalId", skip_serializing_if = "Option::is_none")]
    pub withdrawal_id: Option<String>,
}

impl PayoutState {
    pub fn new(assets: crate::models::Assets, contract_id: String, payout_id: String, payout_validator_address: String, role: crate::models::AssetId, status: crate::models::PayoutStatus) -> PayoutState {
        PayoutState {
            assets: Box::new(assets),
            contract_id,
            payout_id,
            payout_validator_address,
            role: Box::new(role),
            status,
            withdrawal_id: None,
        }
    }
}


