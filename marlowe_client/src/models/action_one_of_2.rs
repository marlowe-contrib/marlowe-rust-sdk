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
pub struct ActionOneOf2 {
    #[serde(rename = "notify_if")]
    pub notify_if: Box<crate::models::Observation>,
}

impl ActionOneOf2 {
    pub fn new(notify_if: crate::models::Observation) -> ActionOneOf2 {
        ActionOneOf2 {
            notify_if: Box::new(notify_if),
        }
    }
}


