/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{PayToAccountObject, PayToPartyObject};

/// PayeeObject : A recipient of a payment

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayeeObject {
    PayToAccountObject(PayToAccountObject),
    PayToPartyObject(PayToPartyObject),
}
