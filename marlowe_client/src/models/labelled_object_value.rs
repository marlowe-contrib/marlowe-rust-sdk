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
pub struct LabelledObjectValue {
    #[serde(rename = "amount_of_token")]
    pub amount_of_token: Box<crate::models::TokenObject>,
    #[serde(rename = "in_account")]
    pub in_account: Box<crate::models::PartyObject>,
    #[serde(rename = "negate")]
    pub negate: Box<crate::models::ValueObject>,
    #[serde(rename = "add")]
    pub add: Box<crate::models::ValueObject>,
    #[serde(rename = "and")]
    pub and: Box<crate::models::ObservationObject>,
    #[serde(rename = "minus")]
    pub minus: Box<crate::models::ValueObject>,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ValueObject>,
    #[serde(rename = "multiply")]
    pub multiply: Box<crate::models::ValueObject>,
    #[serde(rename = "times")]
    pub times: Box<crate::models::ValueObject>,
    #[serde(rename = "by")]
    pub by: Box<crate::models::ValueObject>,
    #[serde(rename = "divide")]
    pub divide: Box<crate::models::ValueObject>,
    #[serde(rename = "value_of_choice")]
    pub value_of_choice: Box<crate::models::ChoiceIdObject>,
    #[serde(rename = "use_value")]
    pub use_value: String,
    #[serde(rename = "else")]
    pub r#else: Box<crate::models::ContractObject>,
    #[serde(rename = "if")]
    pub r#if: Box<crate::models::ObservationObject>,
    #[serde(rename = "then")]
    pub then: Box<crate::models::ContractObject>,
    /// An arbitrary text identifier for an object in a Marlowe object bundle.
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "both")]
    pub both: Box<crate::models::ObservationObject>,
    #[serde(rename = "either")]
    pub either: Box<crate::models::ObservationObject>,
    #[serde(rename = "or")]
    pub or: Box<crate::models::ObservationObject>,
    #[serde(rename = "not")]
    pub not: Box<crate::models::ObservationObject>,
    #[serde(rename = "chose_something_for")]
    pub chose_something_for: Box<crate::models::ChoiceIdObject>,
    #[serde(rename = "ge_than")]
    pub ge_than: Box<crate::models::ValueObject>,
    #[serde(rename = "gt")]
    pub gt: Box<crate::models::ValueObject>,
    #[serde(rename = "lt")]
    pub lt: Box<crate::models::ValueObject>,
    #[serde(rename = "le_than")]
    pub le_than: Box<crate::models::ValueObject>,
    #[serde(rename = "equal_to")]
    pub equal_to: Box<crate::models::ValueObject>,
    #[serde(rename = "from_account")]
    pub from_account: Box<crate::models::PartyObject>,
    #[serde(rename = "pay")]
    pub pay: Box<crate::models::ValueObject>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::PayeeObject>,
    #[serde(rename = "token")]
    pub token: Box<crate::models::TokenObject>,
    #[serde(rename = "timeout")]
    pub timeout: i32,
    #[serde(rename = "timeout_continuation")]
    pub timeout_continuation: Box<crate::models::ContractObject>,
    #[serde(rename = "when")]
    pub when: Vec<crate::models::CaseObject>,
    #[serde(rename = "be")]
    pub be: Box<crate::models::ValueObject>,
    #[serde(rename = "let")]
    pub r#let: String,
    #[serde(rename = "assert")]
    pub assert: Box<crate::models::ObservationObject>,
    #[serde(rename = "role_token")]
    pub role_token: String,
    /// A cardano address
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "currency_symbol")]
    pub currency_symbol: String,
    #[serde(rename = "token_name")]
    pub token_name: String,
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
}

impl LabelledObjectValue {
    pub fn new(amount_of_token: crate::models::TokenObject, in_account: crate::models::PartyObject, negate: crate::models::ValueObject, add: crate::models::ValueObject, and: crate::models::ObservationObject, minus: crate::models::ValueObject, value: crate::models::ValueObject, multiply: crate::models::ValueObject, times: crate::models::ValueObject, by: crate::models::ValueObject, divide: crate::models::ValueObject, value_of_choice: crate::models::ChoiceIdObject, use_value: String, r#else: crate::models::ContractObject, r#if: crate::models::ObservationObject, then: crate::models::ContractObject, r#ref: String, both: crate::models::ObservationObject, either: crate::models::ObservationObject, or: crate::models::ObservationObject, not: crate::models::ObservationObject, chose_something_for: crate::models::ChoiceIdObject, ge_than: crate::models::ValueObject, gt: crate::models::ValueObject, lt: crate::models::ValueObject, le_than: crate::models::ValueObject, equal_to: crate::models::ValueObject, from_account: crate::models::PartyObject, pay: crate::models::ValueObject, to: crate::models::PayeeObject, token: crate::models::TokenObject, timeout: i32, timeout_continuation: crate::models::ContractObject, when: Vec<crate::models::CaseObject>, be: crate::models::ValueObject, r#let: String, assert: crate::models::ObservationObject, role_token: String, address: String, currency_symbol: String, token_name: String, deposits: crate::models::Value, into_account: crate::models::Party, of_token: crate::models::Token, party: crate::models::Party, choose_between: Vec<crate::models::Bound>, for_choice: crate::models::ChoiceId, notify_if: crate::models::Observation) -> LabelledObjectValue {
        LabelledObjectValue {
            amount_of_token: Box::new(amount_of_token),
            in_account: Box::new(in_account),
            negate: Box::new(negate),
            add: Box::new(add),
            and: Box::new(and),
            minus: Box::new(minus),
            value: Box::new(value),
            multiply: Box::new(multiply),
            times: Box::new(times),
            by: Box::new(by),
            divide: Box::new(divide),
            value_of_choice: Box::new(value_of_choice),
            use_value,
            r#else: Box::new(r#else),
            r#if: Box::new(r#if),
            then: Box::new(then),
            r#ref,
            both: Box::new(both),
            either: Box::new(either),
            or: Box::new(or),
            not: Box::new(not),
            chose_something_for: Box::new(chose_something_for),
            ge_than: Box::new(ge_than),
            gt: Box::new(gt),
            lt: Box::new(lt),
            le_than: Box::new(le_than),
            equal_to: Box::new(equal_to),
            from_account: Box::new(from_account),
            pay: Box::new(pay),
            to: Box::new(to),
            token: Box::new(token),
            timeout,
            timeout_continuation: Box::new(timeout_continuation),
            when,
            be: Box::new(be),
            r#let,
            assert: Box::new(assert),
            role_token,
            address,
            currency_symbol,
            token_name,
            deposits: Box::new(deposits),
            into_account: Box::new(into_account),
            of_token: Box::new(of_token),
            party: Box::new(party),
            choose_between,
            for_choice: Box::new(for_choice),
            notify_if: Box::new(notify_if),
        }
    }
}


