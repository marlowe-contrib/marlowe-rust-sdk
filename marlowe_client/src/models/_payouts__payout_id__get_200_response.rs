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
pub struct PayoutsPayoutIdGet200Response {
    #[serde(rename = "links")]
    pub links: Box<crate::models::PayoutsPayoutIdGet200ResponseLinks>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::PayoutState>,
}

impl PayoutsPayoutIdGet200Response {
    pub fn new(links: crate::models::PayoutsPayoutIdGet200ResponseLinks, resource: crate::models::PayoutState) -> PayoutsPayoutIdGet200Response {
        PayoutsPayoutIdGet200Response {
            links: Box::new(links),
            resource: Box::new(resource),
        }
    }
}

