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
pub struct IfValue {
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::Value>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::Observation>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Value>,
}

impl IfValue {
    pub fn new(
        r#else: crate::models::Value,
        r#if: crate::models::Observation,
        then: crate::models::Value,
    ) -> IfValue {
        IfValue {
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            then: Box::new(then),
        }
    }
}
