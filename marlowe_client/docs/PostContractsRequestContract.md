# PostContractsRequestContract

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_account** | [**crate::models::Party**](Party.md) |  | 
**pay** | [**crate::models::Value**](Value.md) |  | 
**then** | [**crate::models::Contract**](Contract.md) |  | 
**to** | [**crate::models::Payee**](Payee.md) |  | 
**token** | [**crate::models::Token**](Token.md) |  | 
**r#else** | [**crate::models::Contract**](Contract.md) |  | 
**r#if** | [**crate::models::Observation**](Observation.md) |  | 
**timeout** | **i64** |  | 
**timeout_continuation** | [**crate::models::Contract**](Contract.md) |  | 
**when** | [**Vec<crate::models::Case>**](Case.md) |  | 
**be** | [**crate::models::Value**](Value.md) |  | 
**r#let** | **String** |  | 
**assert** | [**crate::models::Observation**](Observation.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


