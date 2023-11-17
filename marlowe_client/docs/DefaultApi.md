# \DefaultApi

All URIs are relative to *https://marlowe-runtime-preprod-web.scdev.aws.iohkdev.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_inputs_to_contract**](DefaultApi.md#apply_inputs_to_contract) | **POST** /contracts/{contractId}/transactions | Apply inputs to contract
[**create_contract**](DefaultApi.md#create_contract) | **POST** /contracts | Create a new contract
[**create_contract_sources**](DefaultApi.md#create_contract_sources) | **POST** /contracts/sources | Upload contract sources
[**get_contract_by_id**](DefaultApi.md#get_contract_by_id) | **GET** /contracts/{contractId} | Get contract by ID
[**get_contract_source_adjacency**](DefaultApi.md#get_contract_source_adjacency) | **GET** /contracts/sources/{contractSourceId}/adjacency | Get adjacent contract source IDs by ID
[**get_contract_source_by_id**](DefaultApi.md#get_contract_source_by_id) | **GET** /contracts/sources/{contractSourceId} | Get contract source by ID
[**get_contract_source_closure**](DefaultApi.md#get_contract_source_closure) | **GET** /contracts/sources/{contractSourceId}/closure | Get contract source closure by ID
[**get_contract_transaction_by_id**](DefaultApi.md#get_contract_transaction_by_id) | **GET** /contracts/{contractId}/transactions/{transactionId} | Get contract transaction by ID
[**get_contracts**](DefaultApi.md#get_contracts) | **GET** /contracts | Get contracts
[**get_next_steps_for_contract**](DefaultApi.md#get_next_steps_for_contract) | **GET** /contracts/{contractId}/next | Get next contract steps
[**get_payout_by_id**](DefaultApi.md#get_payout_by_id) | **GET** /payouts/{payoutId} | Get payout by ID
[**get_payouts**](DefaultApi.md#get_payouts) | **GET** /payouts | Get role payouts
[**get_transactions_for_contract**](DefaultApi.md#get_transactions_for_contract) | **GET** /contracts/{contractId}/transactions | Get transactions for contract
[**get_withdrawal_by_id**](DefaultApi.md#get_withdrawal_by_id) | **GET** /withdrawals/{withdrawalId} | Get withdrawal by ID
[**get_withdrawals**](DefaultApi.md#get_withdrawals) | **GET** /withdrawals | Get withdrawals
[**healthcheck**](DefaultApi.md#healthcheck) | **GET** /healthcheck | Test server status
[**submit_contract**](DefaultApi.md#submit_contract) | **PUT** /contracts/{contractId} | Submit contract to chain
[**submit_contract_transaction**](DefaultApi.md#submit_contract_transaction) | **PUT** /contracts/{contractId}/transactions/{transactionId} | Submit contract input application
[**submit_withdrawal**](DefaultApi.md#submit_withdrawal) | **PUT** /withdrawals/{withdrawalId} | Submit payout withdrawal
[**withdraw_payouts**](DefaultApi.md#withdraw_payouts) | **POST** /withdrawals | Withdraw payouts



## apply_inputs_to_contract

> crate::models::ApplyInputsResponse apply_inputs_to_contract(contract_id, x_change_address, x_address, x_collateral_utx_o, post_transactions_request)
Apply inputs to contract

Build an unsigned (Cardano) transaction body which applies inputs to an open Marlowe contract. This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted. To submit the signed transaction, use the PUT /contracts/{contractId}/transactions/{transactionId} endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**x_change_address** | **String** |  | [required] |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_transactions_request** | Option<[**PostTransactionsRequest**](PostTransactionsRequest.md)> |  |  |

### Return type

[**crate::models::ApplyInputsResponse**](ApplyInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.apply-inputs-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_contract

> crate::models::CreateContractResponse create_contract(x_change_address, x_stake_address, x_address, x_collateral_utx_o, post_contracts_request)
Create a new contract

Build an unsigned (Cardano) transaction body which opens a new Marlowe contract. This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted. To submit the signed transaction, use the PUT /contracts/{contractId} endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_change_address** | **String** |  | [required] |
**x_stake_address** | Option<**String**> | Where to send staking rewards for the Marlowe script outputs of this contract. |  |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_contracts_request** | Option<[**PostContractsRequest**](PostContractsRequest.md)> |  |  |

### Return type

[**crate::models::CreateContractResponse**](CreateContractResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.contract-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_contract_sources

> crate::models::PostContractSourceResponse create_contract_sources(main, labelled_object)
Upload contract sources

Upload a bundle of marlowe objects as contract sources. This API supports request body streaming, with newline framing between request bundles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main** | **String** | The label of the top-level contract object in the bundle(s). | [required] |
**labelled_object** | Option<[**Vec<crate::models::LabelledObject>**](LabelledObject.md)> |  |  |

### Return type

[**crate::models::PostContractSourceResponse**](PostContractSourceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/json;charset=utf-8
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_by_id

> crate::models::GetContractResponse get_contract_by_id(contract_id)
Get contract by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |

### Return type

[**crate::models::GetContractResponse**](GetContractResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_source_adjacency

> crate::models::ContractSourceIds get_contract_source_adjacency(contract_source_id)
Get adjacent contract source IDs by ID

Get the contract source IDs which are adjacent to a contract source (they appear directly in the contract source).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_source_id** | **String** |  | [required] |

### Return type

[**crate::models::ContractSourceIds**](ContractSourceIds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_source_by_id

> crate::models::Contract get_contract_source_by_id(contract_source_id, expand)
Get contract source by ID

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


## get_contract_source_closure

> crate::models::ContractSourceIds get_contract_source_closure(contract_source_id)
Get contract source closure by ID

Get the contract source IDs which appear in the full hierarchy of a contract source (including the ID of the contract source its self).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_source_id** | **String** |  | [required] |

### Return type

[**crate::models::ContractSourceIds**](ContractSourceIds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contract_transaction_by_id

> crate::models::GetTransactionResponse get_contract_transaction_by_id(contract_id, transaction_id)
Get contract transaction by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**transaction_id** | **String** |  | [required] |

### Return type

[**crate::models::GetTransactionResponse**](GetTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts

> crate::models::GetContractsResponse get_contracts(role_currency, tag, party_address, party_role, range)
Get contracts

Get contracts published on chain. Results are returned in pages, with paging being specified by request headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_currency** | Option<[**Vec<String>**](String.md)> |  |  |
**tag** | Option<[**Vec<String>**](String.md)> |  |  |
**party_address** | Option<[**Vec<String>**](String.md)> |  |  |
**party_role** | Option<[**Vec<String>**](String.md)> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::GetContractsResponse**](GetContractsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_next_steps_for_contract

> crate::models::Next get_next_steps_for_contract(contract_id, validity_start, validity_end, party)
Get next contract steps

Get inputs which could be performed on a contract withing a time range by the requested parties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**validity_start** | **String** | The beginning of the validity range. | [required] |
**validity_end** | **String** | The end of the validity range. | [required] |
**party** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::Next**](Next.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payout_by_id

> crate::models::GetPayoutResponse get_payout_by_id(payout_id)
Get payout by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** |  | [required] |

### Return type

[**crate::models::GetPayoutResponse**](GetPayoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payouts

> crate::models::GetPayoutsResponse get_payouts(contract_id, role_token, status, range)
Get role payouts

Get payouts to parties from role-based contracts. Results are returned in pages, with paging being specified by request headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | Option<[**Vec<String>**](String.md)> |  |  |
**role_token** | Option<[**Vec<String>**](String.md)> |  |  |
**status** | Option<**String**> | Whether to include available or withdrawn payouts in the results. |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::GetPayoutsResponse**](GetPayoutsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_for_contract

> crate::models::GetTransactionsResponse get_transactions_for_contract(contract_id, range)
Get transactions for contract

Get published transactions for a contract. Results are returned in pages, with paging being specified by request headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** |  | [required] |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::GetTransactionsResponse**](GetTransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_withdrawal_by_id

> crate::models::Withdrawal get_withdrawal_by_id(withdrawal_id)
Get withdrawal by ID

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


## get_withdrawals

> crate::models::GetWithdrawalsResponse get_withdrawals(role_currency, range)
Get withdrawals

Get published withdrawal transactions. Results are returned in pages, with paging being specified by request headers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_currency** | Option<[**Vec<String>**](String.md)> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**crate::models::GetWithdrawalsResponse**](GetWithdrawalsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthcheck

> healthcheck()
Test server status

Check if the server is running and ready to respond to requests.

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


## submit_contract

> submit_contract(contract_id, text_envelope)
Submit contract to chain

Submit a signed (Cardano) transaction that opens a new Marlowe contract. The transaction must have originally been created by the POST /contracts endpoint. This endpoint will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block. Use the GET /contracts/{contractId} endpoint to poll the on-chain status.

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


## submit_contract_transaction

> submit_contract_transaction(contract_id, transaction_id, text_envelope)
Submit contract input application

Submit a signed (Cardano) transaction that applies inputs to an open Marlowe contract. The transaction must have originally been created by the POST /contracts/{contractId}/transactions endpoint. This endpoint will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block. Use the GET /contracts/{contractId}/transactions/{transactionId} endpoint to poll the on-chain status.

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


## submit_withdrawal

> submit_withdrawal(withdrawal_id, text_envelope)
Submit payout withdrawal

Submit a signed (Cardano) transaction that withdraws available payouts from a role payout validator. The transaction must have originally been created by the POST /withdrawals endpoint. This endpoint will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block. Use the GET /withdrawals/{withdrawalId} endpoint to poll the on-chain status.

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


## withdraw_payouts

> crate::models::WithdrawPayoutsResponse withdraw_payouts(x_change_address, x_address, x_collateral_utx_o, post_withdrawals_request)
Withdraw payouts

Build an unsigned (Cardano) transaction body which withdraws available payouts from a role payout validator. This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted. To submit the signed transaction, use the PUT /withdrawals/{withdrawalId} endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_change_address** | **String** |  | [required] |
**x_address** | Option<**String**> |  |  |
**x_collateral_utx_o** | Option<**String**> |  |  |
**post_withdrawals_request** | Option<[**PostWithdrawalsRequest**](PostWithdrawalsRequest.md)> |  |  |

### Return type

[**crate::models::WithdrawPayoutsResponse**](WithdrawPayoutsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json;charset=utf-8
- **Accept**: application/vendor.iog.marlowe-runtime.withdraw-tx-json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

