/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Transaction : Information about a Marlowe transaction.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "contract")]
    pub contract: Box<crate::models::Contract>,
    #[serde(rename = "input")]
    pub input: Box<crate::models::TransactionInput>,
    #[serde(rename = "output")]
    pub output: Box<crate::models::TransactionOutput>,
    #[serde(rename = "state")]
    pub state: Box<crate::models::MarloweState>,
}

impl Transaction {
    /// Information about a Marlowe transaction.
    pub fn new(contract: crate::models::Contract, input: crate::models::TransactionInput, output: crate::models::TransactionOutput, state: crate::models::MarloweState) -> Transaction {
        Transaction {
            contract: Box::new(contract),
            input: Box::new(input),
            output: Box::new(output),
            state: Box::new(state),
        }
    }
}


