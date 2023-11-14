/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionInputTxInterval : Time interval.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionInputTxInterval {
    #[serde(rename = "from")]
    pub from: i32,
    #[serde(rename = "to")]
    pub to: i32,
}

impl TransactionInputTxInterval {
    /// Time interval.
    pub fn new(from: i32, to: i32) -> TransactionInputTxInterval {
        TransactionInputTxInterval {
            from,
            to,
        }
    }
}

