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
pub struct CaseThenObject {
    #[serde(rename = "case")]
    pub case: Box<crate::models::ActionObject>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
}

impl CaseThenObject {
    pub fn new(
        case: crate::models::ActionObject,
        then: crate::models::ContractObject,
    ) -> CaseThenObject {
        CaseThenObject {
            case: Box::new(case),
            then: Box::new(then),
        }
    }
}
