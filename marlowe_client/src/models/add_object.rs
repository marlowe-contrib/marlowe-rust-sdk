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
pub struct AddObject {
    #[serde(rename = "add")]
    pub add: Box<crate::models::ValueObject>,
    #[serde(rename = "and")]
    pub and: Box<crate::models::ValueObject>,
}

impl AddObject {
    pub fn new(add: crate::models::ValueObject, and: crate::models::ValueObject) -> AddObject {
        AddObject {
            add: Box::new(add),
            and: Box::new(and),
        }
    }
}
