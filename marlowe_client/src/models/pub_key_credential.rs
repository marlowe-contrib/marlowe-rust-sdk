/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PubKeyCredential : A Plutus public key credential.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PubKeyCredential {
    #[serde(rename = "pubKeyCredential")]
    pub pub_key_credential: String,
}

impl PubKeyCredential {
    /// A Plutus public key credential.
    pub fn new(pub_key_credential: String) -> PubKeyCredential {
        PubKeyCredential {
            pub_key_credential,
        }
    }
}


