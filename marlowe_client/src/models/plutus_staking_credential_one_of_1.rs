/*
 * Marlowe Runtime REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlutusStakingCredentialOneOf1 : A Plutus staking pointer.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlutusStakingCredentialOneOf1 {
    #[serde(rename = "stakingHash")]
    pub staking_hash: Vec<crate::models::PlutusStakingCredentialOneOf1StakingHashInner>,
}

impl PlutusStakingCredentialOneOf1 {
    /// A Plutus staking pointer.
    pub fn new(staking_hash: Vec<crate::models::PlutusStakingCredentialOneOf1StakingHashInner>) -> PlutusStakingCredentialOneOf1 {
        PlutusStakingCredentialOneOf1 {
            staking_hash,
        }
    }
}


