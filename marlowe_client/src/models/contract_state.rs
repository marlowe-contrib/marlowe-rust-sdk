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
pub struct ContractState {
    #[serde(rename = "assets")]
    pub assets: Box<crate::models::Assets>,
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::BlockHeader>>,
    #[serde(rename = "continuations", skip_serializing_if = "Option::is_none")]
    pub continuations: Option<String>,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "currentContract", skip_serializing_if = "Option::is_none")]
    pub current_contract: Option<Box<crate::models::Contract>>,
    #[serde(rename = "initialContract")]
    pub initial_contract: Box<crate::models::Contract>,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, crate::models::Metadata>,
    /// The hex-encoded minting policy ID for a native Cardano token
    #[serde(rename = "roleTokenMintingPolicyId")]
    pub role_token_minting_policy_id: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::MarloweState>>,
    #[serde(rename = "status")]
    pub status: crate::models::TxStatus,
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, crate::models::Metadata>,
    #[serde(rename = "txBody", skip_serializing_if = "Option::is_none")]
    pub tx_body: Option<Box<crate::models::TextEnvelope>>,
    #[serde(rename = "unclaimedPayouts")]
    pub unclaimed_payouts: Vec<crate::models::Payout>,
    /// A reference to a transaction output with a transaction ID and index.
    #[serde(rename = "utxo", skip_serializing_if = "Option::is_none")]
    pub utxo: Option<String>,
    #[serde(rename = "version")]
    pub version: crate::models::MarloweVersion,
}

impl ContractState {
    pub fn new(
        assets: crate::models::Assets,
        contract_id: String,
        initial_contract: crate::models::Contract,
        metadata: ::std::collections::HashMap<String, crate::models::Metadata>,
        role_token_minting_policy_id: String,
        status: crate::models::TxStatus,
        tags: ::std::collections::HashMap<String, crate::models::Metadata>,
        unclaimed_payouts: Vec<crate::models::Payout>,
        version: crate::models::MarloweVersion,
    ) -> ContractState {
        ContractState {
            assets: Box::new(assets),
            block: None,
            continuations: None,
            contract_id,
            current_contract: None,
            initial_contract: Box::new(initial_contract),
            metadata,
            role_token_minting_policy_id,
            state: None,
            status,
            tags,
            tx_body: None,
            unclaimed_payouts,
            utxo: None,
            version,
        }
    }
}
