/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Let : Bind a value to a name within the scope of a sub-contract.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Let {
    #[serde(rename = "be")]
    pub be: Box<crate::models::Value>,
    #[serde(rename = "let")]
    pub r#let: String,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Contract>,
}

impl Let {
    /// Bind a value to a name within the scope of a sub-contract.
    pub fn new(be: crate::models::Value, r#let: String, then: crate::models::Contract) -> Let {
        Let {
            be: Box::new(be),
            r#let,
            then: Box::new(then),
        }
    }
}


