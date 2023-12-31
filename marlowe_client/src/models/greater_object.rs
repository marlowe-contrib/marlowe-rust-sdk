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
pub struct GreaterObject {
    #[serde(rename = "gt")]
    pub gt: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
}

impl GreaterObject {
    pub fn new(gt: crate::models::ValueObject, value: crate::models::ValueObject) -> GreaterObject {
        GreaterObject {
            gt: Box::new(gt),
            value: Box::new(value),
        }
    }
}
