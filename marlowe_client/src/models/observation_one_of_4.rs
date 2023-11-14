/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationOneOf4 {
    #[serde(rename = "ge_than")]
    pub ge_than: Box<crate::models::Value>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::Value>,
}

impl ObservationOneOf4 {
    pub fn new(ge_than: crate::models::Value, value: crate::models::Value) -> ObservationOneOf4 {
        ObservationOneOf4 {
            ge_than: Box::new(ge_than),
            value: Box::new(value),
        }
    }
}


