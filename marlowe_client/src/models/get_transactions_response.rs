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
pub struct GetTransactionsResponse {
    #[serde(rename = "results")]
    pub results: Vec<crate::models::GetTransactionsResponseResultsInner>,
}

impl GetTransactionsResponse {
    pub fn new(results: Vec<crate::models::GetTransactionsResponseResultsInner>) -> GetTransactionsResponse {
        GetTransactionsResponse {
            results,
        }
    }
}


