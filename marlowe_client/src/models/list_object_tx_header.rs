/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListObjectTxHeader {
    #[serde(rename = "results")]
    pub results: Vec<crate::models::ListObjectTxHeaderResultsInner>,
}

impl ListObjectTxHeader {
    pub fn new(results: Vec<crate::models::ListObjectTxHeaderResultsInner>) -> ListObjectTxHeader {
        ListObjectTxHeader {
            results,
        }
    }
}

