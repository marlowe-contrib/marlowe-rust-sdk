/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{CaseMerkleizedThenObject, CaseThenObject};

/// CaseObject : A contract which becomes active when an action occurs.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaseObject {
    CaseThenObject(CaseThenObject),
    CaseMerkleizedThenObject(CaseMerkleizedThenObject),
}
