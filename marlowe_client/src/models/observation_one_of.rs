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
pub struct ObservationOneOf {
    #[serde(rename = "and")]
    pub and: Box<crate::models::Observation>,
    #[serde(rename = "both")]
    pub both: Box<crate::models::Observation>,
}

impl ObservationOneOf {
    pub fn new(and: crate::models::Observation, both: crate::models::Observation) -> ObservationOneOf {
        ObservationOneOf {
            and: Box::new(and),
            both: Box::new(both),
        }
    }
}


