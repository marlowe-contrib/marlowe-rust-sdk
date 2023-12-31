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
pub struct Multiply {
    #[serde(rename = "multiply")]
    pub multiply: Box<crate::models::Value>,
    #[serde(rename = "times")]
    pub times: Box<crate::models::Value>,
}

impl Multiply {
    pub fn new(multiply: crate::models::Value, times: crate::models::Value) -> Multiply {
        Multiply {
            multiply: Box::new(multiply),
            times: Box::new(times),
        }
    }
}
