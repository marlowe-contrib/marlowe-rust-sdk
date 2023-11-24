/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssertFail : A semantics assertion failed.

/// A semantics assertion failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssertFail {
    #[serde(rename = "assertion_failed")]
    AssertionFailed,

}

impl ToString for AssertFail {
    fn to_string(&self) -> String {
        match self {
            Self::AssertionFailed => String::from("assertion_failed"),
        }
    }
}

impl Default for AssertFail {
    fn default() -> AssertFail {
        Self::AssertionFailed
    }
}




