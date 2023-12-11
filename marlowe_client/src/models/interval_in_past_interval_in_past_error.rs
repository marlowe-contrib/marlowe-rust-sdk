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
pub struct IntervalInPastIntervalInPastError {
    #[serde(rename = "from")]
    pub from: i32,
    #[serde(rename = "minTime")]
    pub min_time: i32,
    #[serde(rename = "to")]
    pub to: i32,
}

impl IntervalInPastIntervalInPastError {
    pub fn new(from: i32, min_time: i32, to: i32) -> IntervalInPastIntervalInPastError {
        IntervalInPastIntervalInPastError {
            from,
            min_time,
            to,
        }
    }
}


