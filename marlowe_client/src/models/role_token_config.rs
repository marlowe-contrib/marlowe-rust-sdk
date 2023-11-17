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
pub struct RoleTokenConfig {
    /// A cardano address, in Bech32 format
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::TokenMetadata>,
    /// The type of script receiving the role token.
    #[serde(rename = "script")]
    pub script: Script,
}

impl RoleTokenConfig {
    pub fn new(address: String, metadata: crate::models::TokenMetadata, script: Script) -> RoleTokenConfig {
        RoleTokenConfig {
            address,
            metadata: Box::new(metadata),
            script,
        }
    }
}

/// The type of script receiving the role token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Script {
    #[serde(rename = "ThreadRole")]
    ThreadRole,
    #[serde(rename = "OpenRole")]
    OpenRole,
}

impl Default for Script {
    fn default() -> Script {
        Self::ThreadRole
    }
}

