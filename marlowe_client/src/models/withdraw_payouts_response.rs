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
pub struct WithdrawPayoutsResponse {
    #[serde(rename = "links")]
    pub links: Box<crate::models::GetWithdrawalsResponseResultsInnerLinks>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::WithdrawTxEnvelope>,
}

impl WithdrawPayoutsResponse {
    pub fn new(links: crate::models::GetWithdrawalsResponseResultsInnerLinks, resource: crate::models::WithdrawTxEnvelope) -> WithdrawPayoutsResponse {
        WithdrawPayoutsResponse {
            links: Box::new(links),
            resource: Box::new(resource),
        }
    }
}

