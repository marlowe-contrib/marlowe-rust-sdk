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
pub struct ValueObjectOneOf4 {
    #[serde(rename = "multiply")]
    pub multiply: Box<crate::models::ValueObject>,
    #[serde(rename = "times")]
    pub times: Box<crate::models::ValueObject>,
}

impl ValueObjectOneOf4 {
    pub fn new(multiply: crate::models::ValueObject, times: crate::models::ValueObject) -> ValueObjectOneOf4 {
        ValueObjectOneOf4 {
            multiply: Box::new(multiply),
            times: Box::new(times),
        }
    }
}

