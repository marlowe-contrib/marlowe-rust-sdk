/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Observation : A time-varying expression that evaluates to an integer



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    #[serde(rename = "and")]
    pub and: Box<crate::models::Observation>,
    #[serde(rename = "both")]
    pub both: Box<crate::models::Observation>,
    #[serde(rename = "either")]
    pub either: Box<crate::models::Observation>,
    #[serde(rename = "or")]
    pub or: Box<crate::models::Observation>,
    #[serde(rename = "not")]
    pub not: Box<crate::models::Observation>,
    #[serde(rename = "chose_something_for")]
    pub chose_something_for: Box<crate::models::ChoiceId>,
    #[serde(rename = "ge_than")]
    pub ge_than: Box<crate::models::Value>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::Value>,
    #[serde(rename = "gt")]
    pub gt: Box<crate::models::Value>,
    #[serde(rename = "lt")]
    pub lt: Box<crate::models::Value>,
    #[serde(rename = "le_than")]
    pub le_than: Box<crate::models::Value>,
    #[serde(rename = "equal_to")]
    pub equal_to: Box<crate::models::Value>,
}

impl Observation {
    /// A time-varying expression that evaluates to an integer
    pub fn new(and: crate::models::Observation, both: crate::models::Observation, either: crate::models::Observation, or: crate::models::Observation, not: crate::models::Observation, chose_something_for: crate::models::ChoiceId, ge_than: crate::models::Value, value: crate::models::Value, gt: crate::models::Value, lt: crate::models::Value, le_than: crate::models::Value, equal_to: crate::models::Value) -> Observation {
        Observation {
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
        }
    }
}


