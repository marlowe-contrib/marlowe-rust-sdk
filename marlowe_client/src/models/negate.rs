/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Negate {
    #[serde(rename = "negate")]
    pub negate: Box<crate::models::Value>,
}

impl Negate {
    pub fn new(negate: crate::models::Value) -> Negate {
        Negate {
            negate: Box::new(negate),
        }
    }
}


