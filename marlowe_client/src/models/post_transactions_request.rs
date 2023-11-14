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
pub struct PostTransactionsRequest {
    #[serde(rename = "inputs")]
    pub inputs: Vec<crate::models::Input>,
    #[serde(rename = "invalidBefore", skip_serializing_if = "Option::is_none")]
    pub invalid_before: Option<String>,
    #[serde(rename = "invalidHereafter", skip_serializing_if = "Option::is_none")]
    pub invalid_hereafter: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "version")]
    pub version: crate::models::MarloweVersion,
}

impl PostTransactionsRequest {
    pub fn new(inputs: Vec<crate::models::Input>, metadata: ::std::collections::HashMap<String, serde_json::Value>, tags: ::std::collections::HashMap<String, serde_json::Value>, version: crate::models::MarloweVersion) -> PostTransactionsRequest {
        PostTransactionsRequest {
            inputs,
            invalid_before: None,
            invalid_hereafter: None,
            metadata,
            tags,
            version,
        }
    }
}


