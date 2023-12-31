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
pub struct CaseThen {
    #[serde(rename = "case")]
    pub case: Box<crate::models::Action>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Contract>,
}

impl CaseThen {
    pub fn new(case: crate::models::Action, then: crate::models::Contract) -> CaseThen {
        CaseThen {
            case: Box::new(case),
            then: Box::new(then),
        }
    }
}
