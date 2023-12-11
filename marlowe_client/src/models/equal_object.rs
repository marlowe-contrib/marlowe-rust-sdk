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
pub struct EqualObject {
    #[serde(rename = "equal_to")]
    pub equal_to: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
}

impl EqualObject {
    pub fn new(equal_to: crate::models::ValueObject, value: crate::models::ValueObject) -> EqualObject {
        EqualObject {
            equal_to: Box::new(equal_to),
            value: Box::new(value),
        }
    }
}


