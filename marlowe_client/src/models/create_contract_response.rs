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
pub struct CreateContractResponse {
    #[serde(rename = "links")]
    pub links: Box<crate::models::CreateContractResponseLinks>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::CreateTxEnvelope>,
}

impl CreateContractResponse {
    pub fn new(links: crate::models::CreateContractResponseLinks, resource: crate::models::CreateTxEnvelope) -> CreateContractResponse {
        CreateContractResponse {
            links: Box::new(links),
            resource: Box::new(resource),
        }
    }
}


