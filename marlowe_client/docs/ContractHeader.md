# ContractHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block** | Option<[**crate::models::BlockHeader**](BlockHeader.md)> |  | [optional]
**continuations** | Option<**String**> |  | [optional]
**contract_id** | **String** | A reference to a transaction output with a transaction ID and index. | 
**metadata** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**role_token_minting_policy_id** | **String** | The hex-encoded minting policy ID for a native Cardano token | 
**status** | [**crate::models::TxStatus**](TxStatus.md) |  | 
**tags** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**version** | [**crate::models::MarloweVersion**](MarloweVersion.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


