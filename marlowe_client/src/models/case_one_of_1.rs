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
pub struct CaseOneOf1 {
    #[serde(rename = "case")]
    pub case: Box<crate::models::Action>,
    #[serde(rename = "merkleized_then")]
    pub merkleized_then: String,
}

impl CaseOneOf1 {
    pub fn new(case: crate::models::Action, merkleized_then: String) -> CaseOneOf1 {
        CaseOneOf1 {
            case: Box::new(case),
            merkleized_then,
        }
    }
}


