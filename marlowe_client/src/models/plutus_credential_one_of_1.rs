/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlutusCredentialOneOf1 : A Plutus script credential.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlutusCredentialOneOf1 {
    #[serde(rename = "scriptCredential")]
    pub script_credential: String,
}

impl PlutusCredentialOneOf1 {
    /// A Plutus script credential.
    pub fn new(script_credential: String) -> PlutusCredentialOneOf1 {
        PlutusCredentialOneOf1 {
            script_credential,
        }
    }
}


