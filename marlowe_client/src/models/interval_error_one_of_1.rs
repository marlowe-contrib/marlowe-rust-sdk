/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IntervalErrorOneOf1 : Marlowe transaction interval in past.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntervalErrorOneOf1 {
    #[serde(rename = "intervalInPastError")]
    pub interval_in_past_error: Box<crate::models::IntervalErrorOneOf1IntervalInPastError>,
}

impl IntervalErrorOneOf1 {
    /// Marlowe transaction interval in past.
    pub fn new(interval_in_past_error: crate::models::IntervalErrorOneOf1IntervalInPastError) -> IntervalErrorOneOf1 {
        IntervalErrorOneOf1 {
            interval_in_past_error: Box::new(interval_in_past_error),
        }
    }
}


