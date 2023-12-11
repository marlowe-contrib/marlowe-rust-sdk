/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Bound : An inclusive range of values for a choice.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bound {
    #[serde(rename = "from")]
    pub from: i32,
    #[serde(rename = "to")]
    pub to: i32,
}

impl Bound {
    /// An inclusive range of values for a choice.
    pub fn new(from: i32, to: i32) -> Bound {
        Bound {
            from,
            to,
        }
    }
}


