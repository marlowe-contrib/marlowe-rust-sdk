/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScriptCredential : A Plutus script credential.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptCredential {
    #[serde(rename = "scriptCredential")]
    pub script_credential: String,
}

impl ScriptCredential {
    /// A Plutus script credential.
    pub fn new(script_credential: String) -> ScriptCredential {
        ScriptCredential { script_credential }
    }
}
