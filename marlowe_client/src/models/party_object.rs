/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// PartyObject : A participant in a contract

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartyObject {
    PartyRoleName(crate::models::PartyRoleName),
    PartyAddress(crate::models::PartyAddress),
    LabelRef(crate::models::LabelRef),
}
