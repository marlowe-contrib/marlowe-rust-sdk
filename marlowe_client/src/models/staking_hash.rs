/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// StakingHash : A Plutus staking hash.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StakingHash {
    #[serde(rename = "stakingHash")]
    pub staking_hash: Box<crate::models::PlutusPeriodCredential>,
}

impl StakingHash {
    /// A Plutus staking hash.
    pub fn new(staking_hash: crate::models::PlutusPeriodCredential) -> StakingHash {
        StakingHash {
            staking_hash: Box::new(staking_hash),
        }
    }
}
