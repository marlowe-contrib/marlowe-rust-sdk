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
pub struct PostContractsRequestContract {
    #[serde(rename = "from_account")]
    pub from_account: Box<crate::models::Party>,
    #[serde(rename = "pay")]
    pub pay: Box<crate::models::Value>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::Contract>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::Payee>,
    #[serde(rename = "token")]
    pub token: Box<crate::models::Token>,
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::Contract>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::Observation>,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "timeout_continuation")]
    pub timeout_continuation: Box<crate::models::Contract>,
    #[serde(rename = "when")]
    pub when: Vec<crate::models::Case>,
    #[serde(rename = "be")]
    pub be: Box<crate::models::Value>,
    #[serde(rename = "let")]
    pub r#let: String,
    #[serde(rename = "assert")]
    pub assert: Box<crate::models::Observation>,
}

impl PostContractsRequestContract {
    pub fn new(
        from_account: crate::models::Party,
        pay: crate::models::Value,
        then: crate::models::Contract,
        to: crate::models::Payee,
        token: crate::models::Token,
        r#else: crate::models::Contract,
        r#if: crate::models::Observation,
        timeout: i32,
        timeout_continuation: crate::models::Contract,
        when: Vec<crate::models::Case>,
        be: crate::models::Value,
        r#let: String,
        assert: crate::models::Observation,
    ) -> PostContractsRequestContract {
        PostContractsRequestContract {
            from_account: Box::new(from_account),
            pay: Box::new(pay),
            then: Box::new(then),
            to: Box::new(to),
            token: Box::new(token),
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            timeout,
            timeout_continuation: Box::new(timeout_continuation),
            when,
            be: Box::new(be),
            r#let,
            assert: Box::new(assert),
        }
    }
}
