/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChoiceIdObject : Refers to a party by role name.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChoiceIdObject {
    #[serde(rename = "choice_name")]
    pub choice_name: String,
    #[serde(rename = "choice_owner")]
    pub choice_owner: Box<crate::models::PartyObject>,
}

impl ChoiceIdObject {
    /// Refers to a party by role name.
    pub fn new(choice_name: String, choice_owner: crate::models::PartyObject) -> ChoiceIdObject {
        ChoiceIdObject {
            choice_name,
            choice_owner: Box::new(choice_owner),
        }
    }
}


