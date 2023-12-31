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
pub struct ContractHeader {
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::BlockHeader>>,
    #[serde(rename = "continuations", skip_serializing_if = "Option::is_none")]
    pub continuations: Option<String>,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// The hex-encoded minting policy ID for a native Cardano token
    #[serde(rename = "roleTokenMintingPolicyId")]
    pub role_token_minting_policy_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::TxStatus,
    #[serde(rename = "tags")]
    pub tags: crate::models::Metadata,
    #[serde(rename = "version")]
    pub version: crate::models::MarloweVersion,
}

impl ContractHeader {
    pub fn new(
        contract_id: String,
        metadata: crate::models::Metadata,
        role_token_minting_policy_id: String,
        status: crate::models::TxStatus,
        tags: crate::models::Metadata,
        version: crate::models::MarloweVersion,
    ) -> ContractHeader {
        ContractHeader {
            block: None,
            continuations: None,
            contract_id,
            metadata,
            role_token_minting_policy_id,
            status,
            tags,
            version,
        }
    }
}
