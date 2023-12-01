use std::collections::HashMap;

pub use marlowe_client::models;

use marlowe_client::{
    apis::{
        self,
        configuration::Configuration,
        default_api::GetContractsError,
        default_api::{
            ApplyInputsToContractError, CreateContractError, CreateContractSourcesError,
            GetContractByIdError, GetContractSourceAdjacencyError, GetContractSourceByIdError,
            GetContractSourceClosureError, GetContractTransactionByIdError,
            GetNextStepsForContractError, GetPayoutByIdError, GetPayoutsError,
            GetTransactionsForContractError, GetWithdrawalByIdError, GetWithdrawalsError,
            HealthcheckError, SubmitContractError, SubmitContractTransactionError,
            SubmitWithdrawalError, WithdrawPayoutsError,
        },
        Error,
    },
    models::{
        ApplyInputsTxEnvelope, Contract, ContractHeader, ContractState, CreateTxEnvelope, Next,
        PayoutHeader, PayoutState, PostContractSourceResponse, PostContractsRequest,
        PostTransactionsRequest, PostWithdrawalsRequest, TextEnvelope, Tx, TxHeader,
        WithdrawTxEnvelope, Withdrawal, WithdrawalHeader,
    },
    reqwest::header::HeaderMap,
};

pub type Headers = HashMap<String, String>;

// This function creates the configuration for your API.
// base_path: is the url of the marlowe runtime API server
pub fn init_client(base_path: String) -> Configuration {
    let mut config = Configuration::new();
    config.base_path = base_path;
    config
}

// Transforms the internal representation of headers into a HashMap
fn header_map_to_hash_map(header_map: &HeaderMap) -> HashMap<String, String> {
    header_map
        .iter()
        .map(|(name, value)| {
            // Convert HeaderName and HeaderValue to String
            let name_str = name.as_str().to_string();
            let value_str = value.to_str().unwrap_or_default().to_string();

            (name_str, value_str)
        })
        .collect()
}

// Get contracts published on chain. Results are returned in pages, with paging being specified by request headers.
pub async fn get_contracts(
    configuration: &Configuration,
    role_currency: Option<Vec<String>>,
    tag: Option<Vec<String>>,
    party_address: Option<Vec<String>>,
    party_role: Option<Vec<String>>,
    range: Option<&str>,
) -> Result<(Vec<ContractHeader>, Headers), Error<GetContractsError>> {
    apis::default_api::get_contracts(
        configuration,
        role_currency,
        tag,
        party_address,
        party_role,
        range,
    )
    .await
    .map(|(contracts_data, headers)| {
        let contracts = contracts_data
            .results
            .iter()
            .map(|result| *(result.resource.clone()))
            .collect::<Vec<models::ContractHeader>>();
        (contracts, header_map_to_hash_map(&headers))
    })
}
// Build an unsigned (Cardano) transaction body which opens a new Marlowe contract.
// This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted.
// To submit the signed transaction, use the submit_contract method.
pub async fn create_contract(
    configuration: &Configuration,
    x_change_address: &str,
    x_stake_address: Option<&str>,
    x_address: Option<&str>,
    x_collateral_utx_o: Option<&str>,
    post_contracts_request: Option<PostContractsRequest>,
) -> Result<(CreateTxEnvelope, Headers), Error<CreateContractError>> {
    apis::default_api::create_contract(
        configuration,
        x_change_address,
        x_stake_address,
        x_address,
        x_collateral_utx_o,
        post_contracts_request,
    )
    .await
    .map(|(create_raw_data, headers)| {
        let create_data = *(create_raw_data.resource.clone());
        (create_data, header_map_to_hash_map(&headers))
    })
}

// Upload a bundle of marlowe objects as contract sources.
// This API supports request body streaming, with newline framing between request bundles.
pub async fn upload_contract_sources(
    configuration: &Configuration,
    main: &str,
    labelled_object: Option<Vec<crate::models::LabelledObject>>,
) -> Result<(PostContractSourceResponse, Headers), Error<CreateContractSourcesError>> {
    apis::default_api::create_contract_sources(configuration, main, labelled_object)
        .await
        .map(|(sources_created, headers)| (sources_created, header_map_to_hash_map(&headers)))
}

// Get contract source by ID
pub async fn get_contract_source_by_id(
    configuration: &Configuration,
    contract_source_id: &str,
    expand: Option<bool>,
) -> Result<(Contract, Headers), Error<GetContractSourceByIdError>> {
    apis::default_api::get_contract_source_by_id(configuration, contract_source_id, expand)
        .await
        .map(|(contract, headers)| (contract, header_map_to_hash_map(&headers)))
}

// Get the contract source IDs which are adjacent to a contract source (they appear directly in the contract source).
pub async fn get_contract_source_adjacency(
    configuration: &Configuration,
    contract_source_id: &str,
) -> Result<(Vec<String>, Headers), Error<GetContractSourceAdjacencyError>> {
    apis::default_api::get_contract_source_adjacency(configuration, contract_source_id)
        .await
        .map(|(ids, headers)| (ids.results, header_map_to_hash_map(&headers)))
}

// Get the contract source IDs which appear in the full hierarchy of a contract source (including the ID of the contract source its self).
pub async fn get_contract_source_closure(
    configuration: &Configuration,
    contract_source_id: &str,
) -> Result<(Vec<String>, Headers), Error<GetContractSourceClosureError>> {
    apis::default_api::get_contract_source_closure(configuration, contract_source_id)
        .await
        .map(|(ids, headers)| (ids.results, header_map_to_hash_map(&headers)))
}

// Get contract by ID
pub async fn get_contract_by_id(
    configuration: &Configuration,
    contract_id: &str,
) -> Result<(ContractState, Headers), Error<GetContractByIdError>> {
    apis::default_api::get_contract_by_id(configuration, contract_id)
        .await
        .map(|(contract, headers)| (*contract.resource, header_map_to_hash_map(&headers)))
}

// Submit a signed (Cardano) transaction that opens a new Marlowe contract.
// The transaction must have originally been created by the create_contract function.
// This endpoint will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block.
// get_contract_by_id method to poll the on-chain status.
pub async fn submit_contract(
    configuration: &Configuration,
    contract_id: &str,
    text_envelope: Option<TextEnvelope>,
) -> Result<Headers, Error<SubmitContractError>> {
    apis::default_api::submit_contract(configuration, contract_id, text_envelope)
        .await
        .map(|headers| header_map_to_hash_map(&headers))
}

// Get inputs which could be performed on a contract withing a time range by the requested parties.
pub async fn get_next_steps_for_contract(
    configuration: &Configuration,
    contract_id: &str,
    validity_start: &str,
    validity_end: &str,
    party: Option<Vec<String>>,
) -> Result<(Next, Headers), Error<GetNextStepsForContractError>> {
    apis::default_api::get_next_steps_for_contract(
        configuration,
        contract_id,
        validity_start,
        validity_end,
        party,
    )
    .await
    .map(|(next, headers)| (next, header_map_to_hash_map(&headers)))
}

// Get published transactions for a contract. Results are returned in pages, with paging being specified by request headers.
pub async fn get_transactions_for_contract(
    configuration: &Configuration,
    contract_id: &str,
    range: Option<&str>,
) -> Result<(Vec<TxHeader>, Headers), Error<GetTransactionsForContractError>> {
    apis::default_api::get_transactions_for_contract(configuration, contract_id, range)
        .await
        .map(|(response, headers)| {
            (
                response
                    .results
                    .iter()
                    .map(|result| *(result.resource.clone()))
                    .collect::<Vec<TxHeader>>(),
                header_map_to_hash_map(&headers),
            )
        })
}

// Build an unsigned (Cardano) transaction body which applies inputs to an open Marlowe contract.
// This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted.
// To submit the signed transaction, use the submit_contract_transaction function.
pub async fn apply_inputs_to_contract(
    configuration: &Configuration,
    contract_id: &str,
    x_change_address: &str,
    x_address: Option<&str>,
    x_collateral_utx_o: Option<&str>,
    post_transactions_request: Option<PostTransactionsRequest>,
) -> Result<(ApplyInputsTxEnvelope, Headers), Error<ApplyInputsToContractError>> {
    apis::default_api::apply_inputs_to_contract(
        configuration,
        contract_id,
        x_change_address,
        x_address,
        x_collateral_utx_o,
        post_transactions_request,
    )
    .await
    .map(|(response, headers)| (*(response.resource), header_map_to_hash_map(&headers)))
}

// Get contract transaction by ID
pub async fn get_contract_transaction_by_id(
    configuration: &Configuration,
    contract_id: &str,
    transaction_id: &str,
) -> Result<(Tx, Headers), Error<GetContractTransactionByIdError>> {
    apis::default_api::get_contract_transaction_by_id(configuration, contract_id, transaction_id)
        .await
        .map(|(tx, headers)| (*tx.resource, header_map_to_hash_map(&headers)))
}

// Submit a signed (Cardano) transaction that applies inputs to an open Marlowe contract.
// The transaction must have originally been created by the apply_inputs_to_contract function.
// This function will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block.
// Use the get_contract_transaction_by_id function to poll the on-chain status.
pub async fn submit_contract_transaction(
    configuration: &Configuration,
    contract_id: &str,
    transaction_id: &str,
    text_envelope: Option<TextEnvelope>,
) -> Result<Headers, Error<SubmitContractTransactionError>> {
    apis::default_api::submit_contract_transaction(
        configuration,
        contract_id,
        transaction_id,
        text_envelope,
    )
    .await
    .map(|headers| (header_map_to_hash_map(&headers)))
}

// Check if the server is running and ready to respond to requests.
pub async fn healthcheck(
    configuration: &Configuration,
) -> Result<Headers, Error<HealthcheckError>> {
    apis::default_api::healthcheck(configuration)
        .await
        .map(|headers| header_map_to_hash_map(&headers))
}

// Get payouts to parties from role-based contracts. Results are returned in pages, with paging being specified by request headers.
pub async fn get_payouts(
    configuration: &Configuration,
    contract_id: Option<Vec<String>>,
    role_token: Option<Vec<String>>,
    status: Option<&str>,
    range: Option<&str>,
) -> Result<(Vec<PayoutHeader>, Headers), Error<GetPayoutsError>> {
    apis::default_api::get_payouts(configuration, contract_id, role_token, status, range)
        .await
        .map(|(response, headers)| {
            let payouts = response
                .results
                .iter()
                .map(|result| *(result.resource.clone()))
                .collect::<Vec<PayoutHeader>>();
            (payouts, header_map_to_hash_map(&headers))
        })
}

// Get payout by ID
pub async fn get_payout_by_id(
    configuration: &Configuration,
    payout_id: &str,
) -> Result<(PayoutState, Headers), Error<GetPayoutByIdError>> {
    apis::default_api::get_payout_by_id(configuration, payout_id)
        .await
        .map(|(response, headers)| {
            (
                *(response.resource.clone()),
                header_map_to_hash_map(&headers),
            )
        })
}

// Get published withdrawal transactions. Results are returned in pages, with paging being specified by request headers.
pub async fn get_withdrawals(
    configuration: &Configuration,
    role_currency: Option<Vec<String>>,
    range: Option<&str>,
) -> Result<(Vec<WithdrawalHeader>, Headers), Error<GetWithdrawalsError>> {
    apis::default_api::get_withdrawals(configuration, role_currency, range)
        .await
        .map(|(response, headers)| {
            let withdrawals = response
                .results
                .iter()
                .map(|result| *(result.resource.clone()))
                .collect::<Vec<WithdrawalHeader>>();
            (withdrawals, header_map_to_hash_map(&headers))
        })
}

// Build an unsigned (Cardano) transaction body which withdraws available payouts from a role payout validator.
// This unsigned transaction must be signed by a wallet (such as a CIP-30 or CIP-45 wallet) before being submitted.
// To submit the signed transaction, use the submit_withdrawal function.
pub async fn withdraw_payouts(
    configuration: &Configuration,
    x_change_address: &str,
    x_address: Option<&str>,
    x_collateral_utx_o: Option<&str>,
    post_withdrawals_request: Option<PostWithdrawalsRequest>,
) -> Result<(WithdrawTxEnvelope, Headers), Error<WithdrawPayoutsError>> {
    apis::default_api::withdraw_payouts(
        configuration,
        x_change_address,
        x_address,
        x_collateral_utx_o,
        post_withdrawals_request,
    )
    .await
    .map(|(response, headers)| (*(response.resource), header_map_to_hash_map(&headers)))
}

// Get withdrawal by ID
pub async fn get_withdrawal_by_id(
    configuration: &Configuration,
    withdrawal_id: &str,
) -> Result<(Withdrawal, Headers), Error<GetWithdrawalByIdError>> {
    apis::default_api::get_withdrawal_by_id(configuration, withdrawal_id)
        .await
        .map(|(withdrawal, headers)| ((withdrawal, header_map_to_hash_map(&headers))))
}

// Submit a signed (Cardano) transaction that withdraws available payouts from a role payout validator.
// The transaction must have originally been created by the withdraw_payouts function.
// This function will respond when the transaction is submitted successfully to the local node, which means it will not wait for the transaction to be published in a block.
// Use the get_withdrawal_by_id function to poll the on-chain status.
pub async fn submit_withdrawal(
    configuration: &Configuration,
    withdrawal_id: &str,
    text_envelope: Option<TextEnvelope>,
) -> Result<Headers, Error<SubmitWithdrawalError>> {
    apis::default_api::submit_withdrawal(configuration, withdrawal_id, text_envelope)
        .await
        .map(|headers| (header_map_to_hash_map(&headers)))
}
