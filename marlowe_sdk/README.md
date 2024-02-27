# marlowe-rust-sdk

## Usage

## Config init

```rust
let base_path = String::from("https://marlowe-runtime-preprod-web.scdev.aws.iohkdev.io");
let config = init_client(base_path);
```

## Example of contract creation

```rust
async fn test_create_swap_contract(config: &Configuration) {
    let swap_contract: Contract = Contract::When(When {
        timeout: 2556057600000,
        timeout_continuation: Box::new(Contract::Close(Close::Close)),
        when: vec![Case::Then(CaseThen {
            case: Box::new(Action::DepositAction(DepositAction {
                deposits: Box::new(Value::TokenAmount(3000000)),
                into_account: Box::new(Party::PartyRoleName(PartyRoleName {
                    role_token: "provider".to_owned(),
                })),
                of_token: Box::new(Token {
                    currency_symbol: "".to_owned(),
                    token_name: "".to_owned(),
                }),
                party: Box::new(Party::PartyRoleName(PartyRoleName {
                    role_token: "provider".to_owned(),
                })),
            })),
            then: Box::new(Contract::When(When {
                timeout: 2556057600000,
                timeout_continuation: Box::new(Contract::Pay(Pay {
                    from_account: Box::new(Party::PartyRoleName(PartyRoleName {
                        role_token: "provider".to_owned(),
                    })),
                    pay: Box::new(Value::TokenAmount(3_000_000)),
                    token: Box::new(Token {
                        currency_symbol: "".to_owned(),
                        token_name: "".to_owned(),
                    }),
                    to: Box::new(Payee::PayToParty(PayToParty {
                        party: Box::new(Party::PartyRoleName(PartyRoleName {
                            role_token: "provider".to_owned(),
                        })),
                    })),
                    then: Box::new(Contract::Close(Close::Close)),
                })),
                when: vec![Case::Then(CaseThen {
                    case: Box::new(Action::DepositAction(DepositAction {
                        deposits: Box::new(Value::TokenAmount(3_000_000)),
                        into_account: Box::new(Party::PartyRoleName(PartyRoleName {
                            role_token: "swapper".to_owned(),
                        })),
                        of_token: Box::new(Token {
                            currency_symbol: "".to_owned(),
                            token_name: "".to_owned(),
                        }),
                        party: Box::new(Party::PartyRoleName(PartyRoleName {
                            role_token: "swapper".to_owned(),
                        })),
                    })),
                    then: Box::new(Contract::Pay(Pay {
                        pay: Box::new(Value::TokenAmount(3_000_000)),
                        token: Box::new(Token {
                            currency_symbol: "".to_owned(),
                            token_name: "".to_owned(),
                        }),
                        to: Box::new(Payee::PayToParty(PayToParty {
                            party: Box::new(Party::PartyRoleName(PartyRoleName {
                                role_token: "swapper".to_owned(),
                            })),
                        })),
                        from_account: Box::new(Party::PartyRoleName(PartyRoleName {
                            role_token: "provider".to_owned(),
                        })),
                        then: Box::new(Contract::Pay(Pay {
                            from_account: Box::new(Party::PartyRoleName(PartyRoleName {
                                role_token: "swapper".to_owned(),
                            })),
                            pay: Box::new(Value::TokenAmount(3_000_000)),
                            then: Box::new(Contract::Close(Close::Close)),
                            to: Box::new(Payee::PayToParty(PayToParty {
                                party: Box::new(Party::PartyRoleName(PartyRoleName {
                                    role_token: "provider".to_owned(),
                                })),
                            })),
                            token: Box::new(Token {
                                currency_symbol: "".to_owned(),
                                token_name: "".to_owned(),
                            }),
                        })),
                    })),
                })],
            })),
        })],
    });
    let contract: Box<PostContractsRequestContract> =
        Box::new(PostContractsRequestContract::Contract(swap_contract));
    let metadata = HashMap::new();
    let min_utx_o_deposit = Some(5_000_000);
    let mut roles_hash_map: HashMap<String, RoleTokenConfig> = HashMap::new();
    roles_hash_map.insert(
        "swapper".to_owned(),
        RoleTokenConfig::Address(
            // insert your address here
            String::from("<your address>")
        ),
    );
    roles_hash_map.insert(
        "provider".to_owned(),
        RoleTokenConfig::Address(
            // insert your address here
            String::from("<your address>"),
        ),
    );

    let roles: Box<RolesConfig> = Box::new(RolesConfig::AdditionalRolesConfigProp(roles_hash_map));
    let contract_request: PostContractsRequest = PostContractsRequest {
        contract,
        metadata,
        min_utx_o_deposit,
        roles: Some(roles),
        tags: HashMap::new(),
        version: marlowe_rust_sdk::models::MarloweVersion::V1,
        thread_token_name: None,
    };
    let maybe_contract = marlowe_rust_sdk::create_contract(
        config,
        "<your address>",
        None,
        None,
        None,
        contract_request,
    )
    .await;
    match maybe_contract {
        Ok(contract) => {
            println!("{:#?}", contract);
            println!("Contract created successfully");
        }
        Err(error) => {
            println!("{:#?}", error);
            println!("An error ocurred creating your swap contract");
        }
    }
}
```

## SDK structure

- Wrapper functions for the automatically generated endpoint interactions located in the `src/lib.rs` file.
- Function to initialize client configuration.

## SDK development

- Get OpenAPI specs from the Marlowe Runtime REST API.
- Format the OpenAPI specs manually (i.e., `oneOf` fields are named accordingly, use previous versions to help yourself with the naming of schemas).
- Set the OpenAPI versions of the specs to 3.0.0 which the tool can safely handle.
- One of the most problematic schemas is `MarloweState`, use previous versions to help yourself.
- Use the `openapi-generator-cli` tool to validate the specs and generate the code.
- Run `cargo fmt` to format the code.
- Adapt the schemas that have `oneOf`. Currently (`openapi-generator-cli v7.1.0`) the generator can't create enums out of `oneOf` schemas, you'll have to do that manually.
- Once everything is ready, run `cargo test` to make sure the (getter) functions are working properly.
