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
pub struct ObservationObjectOneOf {
    #[serde(rename = "and")]
    pub and: Box<crate::models::ObservationObject>,
    #[serde(rename = "both")]
    pub both: Box<crate::models::ObservationObject>,
}

impl ObservationObjectOneOf {
    pub fn new(and: crate::models::ObservationObject, both: crate::models::ObservationObject) -> ObservationObjectOneOf {
        ObservationObjectOneOf {
            and: Box::new(and),
            both: Box::new(both),
        }
    }
}

