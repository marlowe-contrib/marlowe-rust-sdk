/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlutusPeriodCredential : A Plutus credential.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlutusPeriodCredential {
    #[serde(rename = "pubKeyCredential")]
    pub pub_key_credential: String,
    #[serde(rename = "scriptCredential")]
    pub script_credential: String,
}

impl PlutusPeriodCredential {
    /// A Plutus credential.
    pub fn new(pub_key_credential: String, script_credential: String) -> PlutusPeriodCredential {
        PlutusPeriodCredential {
            pub_key_credential,
            script_credential,
        }
    }
}

