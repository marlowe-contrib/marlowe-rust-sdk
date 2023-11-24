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
pub struct LesserOrEqual {
    #[serde(rename = "le_than")]
    pub le_than: Box<crate::models::Value>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::Value>,
}

impl LesserOrEqual {
    pub fn new(le_than: crate::models::Value, value: crate::models::Value) -> LesserOrEqual {
        LesserOrEqual {
            le_than: Box::new(le_than),
            value: Box::new(value),
        }
    }
}

