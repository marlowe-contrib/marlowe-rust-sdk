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
pub struct ValueOneOf5 {
    #[serde(rename = "by")]
    pub by: Box<crate::models::Value>,
    #[serde(rename = "divide")]
    pub divide: Box<crate::models::Value>,
}

impl ValueOneOf5 {
    pub fn new(by: crate::models::Value, divide: crate::models::Value) -> ValueOneOf5 {
        ValueOneOf5 {
            by: Box::new(by),
            divide: Box::new(divide),
        }
    }
}

