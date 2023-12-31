/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// Metadata : Arbitrary JSON-encoded transaction metadata

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Metadata {
    Integer(i64),
    HexMetadata(String),
    TextMetadata(String),
    MetadataArray(Vec<Metadata>),
    AdditionalMetadata(std::collections::HashMap<String, Option<Metadata>>),
}
