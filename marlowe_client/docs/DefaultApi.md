# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contracts_contract_id_get**](DefaultApi.md#contracts_contract_id_get) | **GET** /contracts/{contractId} | 
[**contracts_contract_id_next_get**](DefaultApi.md#contracts_contract_id_next_get) | **GET** /contracts/{contractId}/next | 
[**contracts_contract_id_put**](DefaultApi.md#contracts_contract_id_put) | **PUT** /contracts/{contractId} | 
[**contracts_contract_id_transactions_get**](DefaultApi.md#contracts_contract_id_transactions_get) | **GET** /contracts/{contractId}/transactions | 
[**contracts_contract_id_transactions_post**](DefaultApi.md#contracts_contract_id_transactions_post) | **POST** /contracts/{contractId}/transactions | 
[**contracts_contract_id_transactions_transaction_id_get**](DefaultApi.md#contracts_contract_id_transactions_transaction_id_get) | **GET** /contracts/{contractId}/transactions/{transactionId} | 
[**contracts_contract_id_transactions_transaction_id_put**](DefaultApi.md#contracts_contract_id_transactions_transaction_id_put) | **PUT** /contracts/{contractId}/transactions/{transactionId} | 
[**contracts_get**](DefaultApi.md#contracts_get) | **GET** /contracts | 
[**contracts_post**](DefaultApi.md#contracts_post) | **POST** /contracts | 
[**contracts_sources_contract_source_id_adjacency_get**](DefaultApi.md#contracts_sources_contract_source_id_adjacency_get) | **GET** /contracts/sources/{contractSourceId}/adjacency | 
[**contracts_sources_contract_source_id_closure_get**](DefaultApi.md#contracts_sources_contract_source_id_closure_get) | **GET** /contracts/sources/{contractSourceId}/closure | 
[**contracts_sources_contract_source_id_get**](DefaultApi.md#contracts_sources_contract_source_id_get) | **GET** /contracts/sources/{contractSourceId} | 
[**contracts_sources_post**](DefaultApi.md#contracts_sources_post) | **POST** /contracts/sources | 
[**healthcheck_get**](DefaultApi.md#healthcheck_get) | **GET** /healthcheck | 
[**payouts_get**](DefaultApi.md#payouts_get) | **GET** /payouts | 
[**payouts_payout_id_get**](DefaultApi.md#payouts_payout_id_get) | **GET** /payouts/{payoutId} | 
[**withdrawals_get**](DefaultApi.md#withdrawals_get) | **GET** /withdrawals | 
[**withdrawals_post**](DefaultApi.md#withdrawals_post) | **POST** /withdrawals | 
[**withdrawals_withdrawal_id_get**](DefaultApi.md#withdrawals_withdrawal_id_get) | **GET** /withdrawals/{withdrawalId} | 
[**withdrawals_withdrawal_id_put**](DefaultApi.md#withdrawals_withdrawal_id_put) | **PUT** /withdrawals/{withdrawalId} | 



## contracts_contract_id_get

> crate::models::ContractsContractIdGet200Response contracts_contract_id_get(contract_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |

### Return type

[**crate::models::ContractsContractIdGet200Response**](_contracts__contractId__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_next_get

> crate::models::Next contracts_contract_id_next_get(contract_id, validity_start, validity_end, party)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**validity_start** | **String** |  | [required] |
**validity_end** | **String** |  | [required] |
**party** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::Next**](Next.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_put

> contracts_contract_id_put(contract_id, text_envelope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**text_envelope** | Option<[**TextEnvelope**](TextEnvelope.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_transactions_get

> crate::models::ListObjectTxHeader contracts_contract_id_transactions_get(contract_id, range)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::ListObjectTxHeader**](ListObject_TxHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_transactions_post

> crate::models::ContractsContractIdTransactionsPost201Response contracts_contract_id_transactions_post(contract_id, x_change_address, x_address, x_collateral_utx_o, post_transactions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**x_change_address** | **String** |  | [required] |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_transactions_request** | Option<[**PostTransactionsRequest**](PostTransactionsRequest.md)> |  |  |

### Return type

[**crate::models::ContractsContractIdTransactionsPost201Response**](_contracts__contractId__transactions_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.apply-inputs-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_transactions_transaction_id_get

> crate::models::ContractsContractIdTransactionsTransactionIdGet200Response contracts_contract_id_transactions_transaction_id_get(contract_id, transaction_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

[**crate::models::ContractsContractIdTransactionsTransactionIdGet200Response**](_contracts__contractId__transactions__transactionId__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_contract_id_transactions_transaction_id_put

> contracts_contract_id_transactions_transaction_id_put(contract_id, transaction_id, text_envelope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**transaction_id** | **String** |  | [required] |
**text_envelope** | Option<[**TextEnvelope**](TextEnvelope.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_get

> crate::models::ListObjectContractHeader contracts_get(role_currency, tag, party_address, party_role, range)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_currency** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**party_address** | Option<[**Vec<String>**](String.md)> |  |  |
**party_role** | Option<[**Vec<String>**](String.md)> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::ListObjectContractHeader**](ListObject_ContractHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_post

> crate::models::ContractsPost201Response contracts_post(x_change_address, x_stake_address, x_address, x_collateral_utx_o, post_contracts_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_change_address** | **String** |  | [required] |
**x_stake_address** | Option<**String**> |  |  |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_contracts_request** | Option<[**PostContractsRequest**](PostContractsRequest.md)> |  |  |

### Return type

[**crate::models::ContractsPost201Response**](_contracts_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.contract-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_sources_contract_source_id_adjacency_get

> crate::models::ListObjectContractSourceId contracts_sources_contract_source_id_adjacency_get(contract_source_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_source_id** | **String** |  | [required] |

### Return type

[**crate::models::ListObjectContractSourceId**](ListObject_ContractSourceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_sources_contract_source_id_closure_get

> crate::models::ListObjectContractSourceId contracts_sources_contract_source_id_closure_get(contract_source_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_source_id** | **String** |  | [required] |

### Return type

[**crate::models::ListObjectContractSourceId**](ListObject_ContractSourceId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_sources_contract_source_id_get

> crate::models::Contract contracts_sources_contract_source_id_get(contract_source_id, expand)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_source_id** | **String** |  | [required] |
**expand** | Option<**bool**> |  |  |[default to false]

### Return type

[**crate::models::Contract**](Contract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contracts_sources_post

> crate::models::PostContractSourceResponse contracts_sources_post(main, labelled_object)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main** | **String** |  | [required] |
**labelled_object** | Option<[**Vec<crate::models::LabelledObject>**](LabelledObject.md)> |  |  |

### Return type

[**crate::models::PostContractSourceResponse**](PostContractSourceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/json;charset=utf-8
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthcheck_get

> healthcheck_get()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payouts_get

> crate::models::ListObjectPayoutHeader payouts_get(contract_id, role_token, status, range)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | Option<[**Vec<String>**](String.md)> |  |  |
**role_token** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<**String**> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::ListObjectPayoutHeader**](ListObject_PayoutHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payouts_payout_id_get

> crate::models::PayoutsPayoutIdGet200Response payouts_payout_id_get(payout_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** |  | [required] |

### Return type

[**crate::models::PayoutsPayoutIdGet200Response**](_payouts__payoutId__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdrawals_get

> crate::models::ListObjectWithdrawalHeader withdrawals_get(role_currency, range)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_currency** | Option<[**Vec<String>**](String.md)> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::ListObjectWithdrawalHeader**](ListObject_WithdrawalHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdrawals_post

> crate::models::WithdrawalsPost201Response withdrawals_post(x_change_address, x_address, x_collateral_utx_o, post_withdrawals_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_change_address** | **String** |  | [required] |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_withdrawals_request** | Option<[**PostWithdrawalsRequest**](PostWithdrawalsRequest.md)> |  |  |

### Return type

[**crate::models::WithdrawalsPost201Response**](_withdrawals_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.withdraw-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdrawals_withdrawal_id_get

> crate::models::Withdrawal withdrawals_withdrawal_id_get(withdrawal_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**withdrawal_id** | **String** |  | [required] |

### Return type

[**crate::models::Withdrawal**](Withdrawal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdrawals_withdrawal_id_put

> withdrawals_withdrawal_id_put(withdrawal_id, text_envelope)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**withdrawal_id** | **String** |  | [required] |
**text_envelope** | Option<[**TextEnvelope**](TextEnvelope.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

