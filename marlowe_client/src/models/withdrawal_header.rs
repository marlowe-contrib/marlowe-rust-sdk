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
pub struct WithdrawalHeader {
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::BlockHeader>>,
    #[serde(rename = "status")]
    pub status: crate::models::TxStatus,
    /// The hex-encoded identifier of a Cardano transaction
    #[serde(rename = "withdrawalId")]
    pub withdrawal_id: String,
}

impl WithdrawalHeader {
    pub fn new(status: crate::models::TxStatus, withdrawal_id: String) -> WithdrawalHeader {
        WithdrawalHeader {
            block: None,
            status,
            withdrawal_id,
        }
    }
}
