/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Case : A contract which becomes active when an action occurs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Case {
    #[serde(rename = "case")]
    pub case: Box<crate::models::Action>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Contract>,
    #[serde(rename = "merkleized_then")]
    pub merkleized_then: String,
}

impl Case {
    /// A contract which becomes active when an action occurs.
    pub fn new(case: crate::models::Action, then: crate::models::Contract, merkleized_then: String) -> Case {
        Case {
            case: Box::new(case),
            then: Box::new(then),
            merkleized_then,
        }
    }
}


