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
pub struct ListObjectContractHeaderResultsInner {
    #[serde(rename = "links")]
    pub links: Box<crate::models::ListObjectContractHeaderResultsInnerLinks>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::ContractHeader>,
}

impl ListObjectContractHeaderResultsInner {
    pub fn new(links: crate::models::ListObjectContractHeaderResultsInnerLinks, resource: crate::models::ContractHeader) -> ListObjectContractHeaderResultsInner {
        ListObjectContractHeaderResultsInner {
            links: Box::new(links),
            resource: Box::new(resource),
        }
    }
}

