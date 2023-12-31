/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// If : If an observation is true, the first contract applies, otherwise the second contract applies.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct If {
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::Contract>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::Observation>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Contract>,
}

impl If {
    /// If an observation is true, the first contract applies, otherwise the second contract applies.
    pub fn new(
        r#else: crate::models::Contract,
        r#if: crate::models::Observation,
        then: crate::models::Contract,
    ) -> If {
        If {
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            then: Box::new(then),
        }
    }
}
