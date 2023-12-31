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
pub struct GetPayoutsResponseResultsInner {
    #[serde(rename = "links")]
    pub links: Box<crate::models::GetPayoutsResponseResultsInnerLinks>,
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::PayoutHeader>,
}

impl GetPayoutsResponseResultsInner {
    pub fn new(
        links: crate::models::GetPayoutsResponseResultsInnerLinks,
        resource: crate::models::PayoutHeader,
    ) -> GetPayoutsResponseResultsInner {
        GetPayoutsResponseResultsInner {
            links: Box::new(links),
            resource: Box::new(resource),
        }
    }
}
