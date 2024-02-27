use marlowe_rust_sdk::{
    get_contract_by_id, get_contract_source_adjacency, get_contract_source_by_id,
    get_contract_source_closure, get_contract_transaction_by_id, get_contracts,
    get_next_steps_for_contract, get_payouts, get_transactions_for_contract, get_withdrawal_by_id,
    get_withdrawals, healthcheck, init_client, Error,
};
use tokio;

const BASE_PATH: &str = "https://marlowe-runtime-preprod-web.scdev.aws.iohkdev.io";
#[test]
fn test_base_path_set() {
    let config = init_client(String::from(BASE_PATH));
    assert_eq!(config.base_path, BASE_PATH);
}

#[tokio::test]
async fn get_contracts_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contracts(&config, None, None, None, None, None).await;
    let successfully_parsed = match res {
        Ok(_) => true,
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        Err(_) => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_contract_source_by_id_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contract_source_by_id(
        &config,
        "705f33bb023b560f458a277c12130487f8dbca1b9e4dc50c4ed1596e00944996",
        Some(true),
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_contract_source_adjacency_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contract_source_adjacency(
        &config,
        "705f33bb023b560f458a277c12130487f8dbca1b9e4dc50c4ed1596e00944996",
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_contract_source_closure_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contract_source_closure(
        &config,
        "705f33bb023b560f458a277c12130487f8dbca1b9e4dc50c4ed1596e00944996",
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_contract_by_id_is_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contract_by_id(
        &config,
        "e74ec98ac5a95f2288bac3eaf563669440f271115634deb8af7db2a463869b00#1",
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_next_steps_for_contract_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_next_steps_for_contract(
        &config,
        // this contract id will not close until 2050
        "26a9d99e3a014b7dafc21642c829b5f51edd8f74f45f13d965e967df182156eb#1",
        "1970-12-06T00:00:00.000Z",
        "2050-01-01T00:00:00.000Z",
        None,
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_transactions_for_contract_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_transactions_for_contract(
        &config,
        // this contract id will not close until 2050
        "06fb28e1322bb2d366617e6fbaed22ed93a8ca2b813964ade5621c4b8fba1ee8#1",
        None,
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_contract_transaction_by_id_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_contract_transaction_by_id(
        &config, // this contract id will not close until 2050
        "06fb28e1322bb2d366617e6fbaed22ed93a8ca2b813964ade5621c4b8fba1ee8#1",
        "981455f49fe566765d8380ad2199ee265ab9128902630780d4d7258a40c9d310",
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn healthcheck_doesnt_panic() {
    let config = init_client(String::from(BASE_PATH));
    let res = healthcheck(&config).await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_payouts_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_payouts(
        &config,
        // this contract id will not close until 2050
        Some(vec![String::from(
            "06fb28e1322bb2d366617e6fbaed22ed93a8ca2b813964ade5621c4b8fba1ee8#1",
        )]),
        None,
        None,
        None,
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_withdrawals_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_withdrawals(&config, None, None).await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}

#[tokio::test]
async fn get_withdrawal_by_id_parsed() {
    let config = init_client(String::from(BASE_PATH));
    let res = get_withdrawal_by_id(
        &config,
        "e68b8034f4d93c4e53468198abdcbe938d067605310ece35ebe681d61c961e1c",
    )
    .await;
    let successfully_parsed = match res {
        Err(Error::Serde(err)) => {
            eprintln!("{:#?}", err);
            false
        }
        _ => true,
    };
    assert!(successfully_parsed);
}
