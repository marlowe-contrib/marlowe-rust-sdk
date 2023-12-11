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
pub struct AssetId {
    #[serde(rename = "assetName")]
    pub asset_name: String,
    /// The hex-encoded minting policy ID for a native Cardano token
    #[serde(rename = "policyId")]
    pub policy_id: String,
}

impl AssetId {
    pub fn new(asset_name: String, policy_id: String) -> AssetId {
        AssetId {
            asset_name,
            policy_id,
        }
    }
}
