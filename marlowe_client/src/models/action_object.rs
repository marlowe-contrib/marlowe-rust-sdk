/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ActionObject : A contract which becomes active when an action occurs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionObject {
    #[serde(rename = "deposits")]
    pub deposits: Box<crate::models::Value>,
    #[serde(rename = "into_account")]
    pub into_account: Box<crate::models::Party>,
    #[serde(rename = "of_token")]
    pub of_token: Box<crate::models::Token>,
    #[serde(rename = "party")]
    pub party: Box<crate::models::Party>,
    #[serde(rename = "choose_between")]
    pub choose_between: Vec<crate::models::Bound>,
    #[serde(rename = "for_choice")]
    pub for_choice: Box<crate::models::ChoiceId>,
    #[serde(rename = "notify_if")]
    pub notify_if: Box<crate::models::Observation>,
    /// An arbitrary text identifier for an object in a Marlowe object bundle.
    #[serde(rename = "ref")]
    pub r#ref: String,
}

impl ActionObject {
    /// A contract which becomes active when an action occurs.
    pub fn new(deposits: crate::models::Value, into_account: crate::models::Party, of_token: crate::models::Token, party: crate::models::Party, choose_between: Vec<crate::models::Bound>, for_choice: crate::models::ChoiceId, notify_if: crate::models::Observation, r#ref: String) -> ActionObject {
        ActionObject {
            deposits: Box::new(deposits),
            into_account: Box::new(into_account),
            of_token: Box::new(of_token),
            party: Box::new(party),
            choose_between,
            for_choice: Box::new(for_choice),
            notify_if: Box::new(notify_if),
            r#ref,
        }
    }
}


