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
pub struct ContractsContractIdTransactionsTransactionIdGet200ResponseLinks {
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
}

impl ContractsContractIdTransactionsTransactionIdGet200ResponseLinks {
    pub fn new() -> ContractsContractIdTransactionsTransactionIdGet200ResponseLinks {
        ContractsContractIdTransactionsTransactionIdGet200ResponseLinks {
            next: None,
            previous: None,
        }
    }
}


