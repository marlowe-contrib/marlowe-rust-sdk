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
pub struct IntervalErrorOneOf1IntervalInPastError {
    #[serde(rename = "from")]
    pub from: i32,
    #[serde(rename = "minTime")]
    pub min_time: i32,
    #[serde(rename = "to")]
    pub to: i32,
}

impl IntervalErrorOneOf1IntervalInPastError {
    pub fn new(from: i32, min_time: i32, to: i32) -> IntervalErrorOneOf1IntervalInPastError {
        IntervalErrorOneOf1IntervalInPastError {
            from,
            min_time,
            to,
        }
    }
}

