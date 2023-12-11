# LabelledObjectValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount_of_token** | [**crate::models::TokenObject**](TokenObject.md) |  | 
**in_account** | [**crate::models::PartyObject**](PartyObject.md) |  | 
**negate** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**add** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**and** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**minus** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**value** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**multiply** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**times** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**by** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**divide** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**value_of_choice** | [**crate::models::ChoiceIdObject**](ChoiceIdObject.md) |  | 
**use_value** | **String** |  | 
**r#else** | [**crate::models::ContractObject**](ContractObject.md) |  | 
**r#if** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**then** | [**crate::models::ContractObject**](ContractObject.md) |  | 
**r#ref** | **String** | An arbitrary text identifier for an object in a Marlowe object bundle. | 
**both** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**either** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**or** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**not** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**chose_something_for** | [**crate::models::ChoiceIdObject**](ChoiceIdObject.md) |  | 
**ge_than** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**gt** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**lt** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**le_than** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**equal_to** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**from_account** | [**crate::models::PartyObject**](PartyObject.md) |  | 
**pay** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**to** | [**crate::models::PayeeObject**](PayeeObject.md) |  | 
**token** | [**crate::models::TokenObject**](TokenObject.md) |  | 
**timeout** | **i64** |  | 
**timeout_continuation** | [**crate::models::ContractObject**](ContractObject.md) |  | 
**when** | [**Vec<crate::models::CaseObject>**](CaseObject.md) |  | 
**be** | [**crate::models::ValueObject**](ValueObject.md) |  | 
**r#let** | **String** |  | 
**assert** | [**crate::models::ObservationObject**](ObservationObject.md) |  | 
**role_token** | **String** |  | 
**address** | **String** | A cardano address, in Bech32 format | 
**currency_symbol** | **String** |  | 
**token_name** | **String** |  | 
**deposits** | [**crate::models::Value**](Value.md) |  | 
**into_account** | [**crate::models::Party**](Party.md) |  | 
**of_token** | [**crate::models::Token**](Token.md) |  | 
**party** | [**crate::models::Party**](Party.md) |  | 
**choose_between** | [**Vec<crate::models::Bound>**](Bound.md) |  | 
**for_choice** | [**crate::models::ChoiceId**](ChoiceId.md) |  | 
**notify_if** | [**crate::models::Observation**](Observation.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


