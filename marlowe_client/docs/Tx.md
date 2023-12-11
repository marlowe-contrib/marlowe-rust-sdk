# Tx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**crate::models::Assets**](Assets.md) |  | 
**block** | Option<[**crate::models::BlockHeader**](BlockHeader.md)> |  | [optional]
**consuming_tx** | Option<**String**> | The hex-encoded identifier of a Cardano transaction | [optional]
**continuations** | Option<**String**> |  | [optional]
**contract_id** | **String** | A reference to a transaction output with a transaction ID and index. | 
**inputs** | [**Vec<crate::models::Input>**](Input.md) |  | 
**input_utxo** | **String** | A reference to a transaction output with a transaction ID and index. | 
**invalid_before** | **String** |  | 
**invalid_hereafter** | **String** |  | 
**metadata** | [**::std::collections::HashMap<String, crate::models::Metadata>**](Metadata.md) |  | 
**output_contract** | Option<[**crate::models::Contract**](Contract.md)> |  | [optional]
**output_state** | Option<[**crate::models::MarloweState**](MarloweState.md)> |  | [optional]
**output_utxo** | Option<**String**> | A reference to a transaction output with a transaction ID and index. | [optional]
**payouts** | [**Vec<crate::models::Payout>**](Payout.md) |  | 
**status** | [**crate::models::TxStatus**](TxStatus.md) |  | 
**tags** | [**::std::collections::HashMap<String, crate::models::Metadata>**](Metadata.md) |  | 
**transaction_id** | **String** | The hex-encoded identifier of a Cardano transaction | 
**tx_body** | Option<[**crate::models::TextEnvelope**](TextEnvelope.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


