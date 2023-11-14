/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ObservationObject : A time-varying expression that evaluates to an integer



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationObject {
    #[serde(rename = "and")]
    pub and: Box<crate::models::ObservationObject>,
    #[serde(rename = "both")]
    pub both: Box<crate::models::ObservationObject>,
    #[serde(rename = "either")]
    pub either: Box<crate::models::ObservationObject>,
    #[serde(rename = "or")]
    pub or: Box<crate::models::ObservationObject>,
    #[serde(rename = "not")]
    pub not: Box<crate::models::ObservationObject>,
    #[serde(rename = "chose_something_for")]
    pub chose_something_for: Box<crate::models::ChoiceIdObject>,
    #[serde(rename = "ge_than")]
    pub ge_than: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
    #[serde(rename = "gt")]
    pub gt: Box<crate::models::ValueObject>,
    #[serde(rename = "lt")]
    pub lt: Box<crate::models::ValueObject>,
    #[serde(rename = "le_than")]
    pub le_than: Box<crate::models::ValueObject>,
    #[serde(rename = "equal_to")]
    pub equal_to: Box<crate::models::ValueObject>,
    /// An arbitrary text identifier for an object in a Marlowe object bundle.
    #[serde(rename = "ref")]
    pub r#ref: String,
}

impl ObservationObject {
    /// A time-varying expression that evaluates to an integer
    pub fn new(and: crate::models::ObservationObject, both: crate::models::ObservationObject, either: crate::models::ObservationObject, or: crate::models::ObservationObject, not: crate::models::ObservationObject, chose_something_for: crate::models::ChoiceIdObject, ge_than: crate::models::ValueObject, value: crate::models::ValueObject, gt: crate::models::ValueObject, lt: crate::models::ValueObject, le_than: crate::models::ValueObject, equal_to: crate::models::ValueObject, r#ref: String) -> ObservationObject {
        ObservationObject {
            and: Box::new(and),
            both: Box::new(both),
            either: Box::new(either),
            or: Box::new(or),
            not: Box::new(not),
            chose_something_for: Box::new(chose_something_for),
            ge_than: Box::new(ge_than),
            value: Box::new(value),
            gt: Box::new(gt),
            lt: Box::new(lt),
            le_than: Box::new(le_than),
            equal_to: Box::new(equal_to),
            r#ref,
        }
    }
}


