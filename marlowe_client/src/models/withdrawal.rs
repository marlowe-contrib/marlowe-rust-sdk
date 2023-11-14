/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Withdrawal {
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::BlockHeader>>,
    #[serde(rename = "payouts")]
    pub payouts: Vec<crate::models::PayoutHeader>,
    #[serde(rename = "status")]
    pub status: crate::models::TxStatus,
    /// The hex-encoded identifier of a Cardano transaction
    #[serde(rename = "withdrawalId")]
    pub withdrawal_id: String,
}

impl Withdrawal {
    pub fn new(payouts: Vec<crate::models::PayoutHeader>, status: crate::models::TxStatus, withdrawal_id: String) -> Withdrawal {
        Withdrawal {
            block: None,
            payouts,
            status,
            withdrawal_id,
        }
    }
}


