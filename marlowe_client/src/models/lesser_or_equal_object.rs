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
pub struct LesserOrEqualObject {
    #[serde(rename = "le_than")]
    pub le_than: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
}

impl LesserOrEqualObject {
    pub fn new(le_than: crate::models::ValueObject, value: crate::models::ValueObject) -> LesserOrEqualObject {
        LesserOrEqualObject {
            le_than: Box::new(le_than),
            value: Box::new(value),
        }
    }
}


