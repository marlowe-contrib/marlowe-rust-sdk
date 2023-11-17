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
pub struct ObservationObjectOneOf3 {
    #[serde(rename = "chose_something_for")]
    pub chose_something_for: Box<crate::models::ChoiceIdObject>,
}

impl ObservationObjectOneOf3 {
    pub fn new(chose_something_for: crate::models::ChoiceIdObject) -> ObservationObjectOneOf3 {
        ObservationObjectOneOf3 {
            chose_something_for: Box::new(chose_something_for),
        }
    }
}


