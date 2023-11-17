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
pub struct GetContractResponseLinks {
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<String>,
}

impl GetContractResponseLinks {
    pub fn new() -> GetContractResponseLinks {
        GetContractResponseLinks {
            transactions: None,
        }
    }
}

