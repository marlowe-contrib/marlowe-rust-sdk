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
pub struct AddressAndMetadata {
    /// A cardano address, in Bech32 format
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::TokenMetadata>,
}

impl AddressAndMetadata {
    pub fn new(address: String) -> AddressAndMetadata {
        AddressAndMetadata {
            address,
            metadata: None,
        }
    }
}
