# TxHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block** | Option<[**crate::models::BlockHeader**](BlockHeader.md)> |  | [optional]
**continuations** | Option<**String**> |  | [optional]
**contract_id** | **String** | A reference to a transaction output with a transaction ID and index. | 
**metadata** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**status** | [**crate::models::TxStatus**](TxStatus.md) |  | 
**tags** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**transaction_id** | **String** | The hex-encoded identifier of a Cardano transaction | 
**utxo** | Option<**String**> | A reference to a transaction output with a transaction ID and index. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


