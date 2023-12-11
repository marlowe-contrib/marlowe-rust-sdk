/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// TokenMetadata : Metadata for an NFT, as described by https://cips.cardano.org/cips/cip25/

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenMetadata {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::TokenMetadataFile>>,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl TokenMetadata {
    /// Metadata for an NFT, as described by https://cips.cardano.org/cips/cip25/
    pub fn new(image: String, name: String) -> TokenMetadata {
        TokenMetadata {
            description: None,
            files: None,
            image,
            media_type: None,
            name,
        }
    }
}
