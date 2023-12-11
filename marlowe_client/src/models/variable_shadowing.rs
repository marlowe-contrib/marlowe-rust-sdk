/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VariableShadowing : A variable-name shadowing warning.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableShadowing {
    #[serde(rename = "had_value")]
    pub had_value: i32,
    #[serde(rename = "is_now_assigned")]
    pub is_now_assigned: i32,
    #[serde(rename = "value_id")]
    pub value_id: String,
}

impl VariableShadowing {
    /// A variable-name shadowing warning.
    pub fn new(had_value: i32, is_now_assigned: i32, value_id: String) -> VariableShadowing {
        VariableShadowing {
            had_value,
            is_now_assigned,
            value_id,
        }
    }
}


