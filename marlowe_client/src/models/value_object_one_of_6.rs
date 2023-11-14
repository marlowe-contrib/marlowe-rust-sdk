/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueObjectOneOf6 {
    #[serde(rename = "value_of_choice")]
    pub value_of_choice: Box<crate::models::ChoiceIdObject>,
}

impl ValueObjectOneOf6 {
    pub fn new(value_of_choice: crate::models::ChoiceIdObject) -> ValueObjectOneOf6 {
        ValueObjectOneOf6 {
            value_of_choice: Box::new(value_of_choice),
        }
    }
}


