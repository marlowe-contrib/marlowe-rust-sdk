# ContractState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**crate::models::Assets**](Assets.md) |  | 
**block** | Option<[**crate::models::BlockHeader**](BlockHeader.md)> |  | [optional]
**continuations** | Option<**String**> |  | [optional]
**contract_id** | **String** | A reference to a transaction output with a transaction ID and index. | 
**current_contract** | Option<[**crate::models::Contract**](Contract.md)> |  | [optional]
**initial_contract** | [**crate::models::Contract**](Contract.md) |  | 
**metadata** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**role_token_minting_policy_id** | **String** | The hex-encoded minting policy ID for a native Cardano token | 
**state** | Option<[**crate::models::MarloweState**](MarloweState.md)> |  | [optional]
**status** | [**crate::models::TxStatus**](TxStatus.md) |  | 
**tags** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**tx_body** | Option<[**crate::models::TextEnvelope**](TextEnvelope.md)> |  | [optional]
**unclaimed_payouts** | [**Vec<crate::models::Payout>**](Payout.md) |  | 
**utxo** | Option<**String**> | A reference to a transaction output with a transaction ID and index. | [optional]
**version** | [**crate::models::MarloweVersion**](MarloweVersion.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


