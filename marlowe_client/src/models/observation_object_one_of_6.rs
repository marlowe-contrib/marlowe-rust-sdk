/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationObjectOneOf6 {
    #[serde(rename = "lt")]
    pub lt: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
}

impl ObservationObjectOneOf6 {
    pub fn new(lt: crate::models::ValueObject, value: crate::models::ValueObject) -> ObservationObjectOneOf6 {
        ObservationObjectOneOf6 {
            lt: Box::new(lt),
            value: Box::new(value),
        }
    }
}


