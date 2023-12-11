/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssertObject : Check an observation and produce a warning if it is false.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssertObject {
    #[serde(rename = "assert")]
    pub assert: Box<crate::models::ObservationObject>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
}

impl AssertObject {
    /// Check an observation and produce a warning if it is false.
    pub fn new(
        assert: crate::models::ObservationObject,
        then: crate::models::ContractObject,
    ) -> AssertObject {
        AssertObject {
            assert: Box::new(assert),
            then: Box::new(then),
        }
    }
}
