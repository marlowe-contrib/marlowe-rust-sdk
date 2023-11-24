/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloseObject : No more payments will be sent and the balance of the contract is 0.

/// No more payments will be sent and the balance of the contract is 0.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloseObject {
    #[serde(rename = "close")]
    Close,

}

impl ToString for CloseObject {
    fn to_string(&self) -> String {
        match self {
            Self::Close => String::from("close"),
        }
    }
}

impl Default for CloseObject {
    fn default() -> CloseObject {
        Self::Close
    }
}




