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
pub struct ContractsContractIdGet200ResponseLinks {
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<String>,
}

impl ContractsContractIdGet200ResponseLinks {
    pub fn new() -> ContractsContractIdGet200ResponseLinks {
        ContractsContractIdGet200ResponseLinks {
            transactions: None,
        }
    }
}


