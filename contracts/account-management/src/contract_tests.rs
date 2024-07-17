use std::collections::HashSet;

use cosmwasm_std::{coin, Addr, BankMsg, Coin, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor, AppBuilder};
use crate::msg::{
    AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SignerListResponse, ThresholdResponse, TransferLimitsResponse, TxExecutionsResponse, WhitelistAddressesResponse
};
use crate::contract::{instantiate, query, execute};
use crate::state::TxStatus;

const ALICE: &str = "alice";
const BOB: &str = "bob";
const CARL: &str = "carl";

const DENOM: &str = "denom";

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked("owner"),
                vec![coin(100000, DENOM)],
            )
            .unwrap();
    })
}

#[test]
fn query_admin() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: ALICE.to_string(),
                signers: vec![CARL.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::Admin{})
        .unwrap();
    assert_eq!(
        resp,
        AdminResponse {
            admin: ALICE.to_string(),
        }
    )
}

#[test]
fn query_signer_list() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: ALICE.to_string(),
                signers: vec![CARL.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: SignerListResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::Signerlist {})
        .unwrap();
    assert_eq!(
        resp,
        SignerListResponse {
            signers: vec![CARL.to_string()],
        }
    )
}

#[test]
fn exec_change_admin() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked( ALICE.to_string()),
            &InstantiateMsg {
                admin: ALICE.to_string(),
                signers: vec![CARL.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Admin {})
        .unwrap();
    assert_eq!(
        resp,
        AdminResponse {
            admin: ALICE.to_string(),
        }
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::ChangeAdmin { 
        new_admin: BOB.to_string(),
    };
    let _ = app
        .execute_contract(
            Addr::unchecked( ALICE.to_string()),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: AdminResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Admin {})
        .unwrap();
    assert_eq!(
        resp,
        AdminResponse {
            admin: BOB.to_string(),
        }
    );
}

#[test]
fn exec_change_threshold() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked( ALICE.to_string()),
            &InstantiateMsg {
                admin: ALICE.to_string(),
                signers: vec![ALICE.to_string(), CARL.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let msg: ExecuteMsg<Empty> = ExecuteMsg::ChangeThreshold { 
        new_threshold: 2,
    };
    let _ = app
        .execute_contract(
            Addr::unchecked( ALICE.to_string()),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: ThresholdResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Threshold {})
        .unwrap();
    assert_eq!(
        resp,
        ThresholdResponse {
            threshold: 2
        }
    );
}

#[test]
fn exec_add_signers() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: SignerListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Signerlist {})
        .unwrap();
    assert_eq!(
        resp,
        SignerListResponse {
            signers: vec![ALICE.to_string()],
        }
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::AddSigners { 
        signers: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: SignerListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Signerlist {})
        .unwrap();
    assert_eq!(
        resp,
        SignerListResponse {
            signers: vec![ALICE.to_string(), BOB.to_string()],
        }
    );
}

#[test]
fn exec_remove_signers() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string(), BOB.to_string()],
                threshold: 1,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: SignerListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Signerlist {})
        .unwrap();
    assert_eq!(
        resp,
        SignerListResponse {
            signers: vec![ALICE.to_string(), BOB.to_string()],
        }
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::RemoveSigners { 
        signers: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: SignerListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Signerlist {})
        .unwrap();
    assert_eq!(
        resp,
        SignerListResponse {
            signers: vec![ALICE.to_string()],
        }
    );
}

#[test]
fn exec_execute_transaction() {
    let mut app = mock_app();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![Addr::unchecked("owner").to_string()],
                threshold: 1,
                whitelist_enabled: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetWhitelistAddresses { 
        addresses: vec![CARL.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetTransferLimits { 
        coins: vec![coin(1100, DENOM)],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();

    let _ = app.send_tokens(Addr::unchecked("owner"), addr.clone(), &[coin(10000, DENOM)]);
    let balance = app.wrap().query_balance(addr.clone(), DENOM).unwrap();
    assert_eq!(balance.amount, Uint128::new(10000));
    assert_eq!(balance.denom, DENOM);
    
    let messages = vec![
        BankMsg::Send {
            to_address: CARL.to_string(),
            amount: vec![coin(1000, DENOM)],
        }
        .into(),
    ];
    let msg: ExecuteMsg<Empty> = ExecuteMsg::ExecuteTransaction {
        msgs: messages,
    };
    let res = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    assert_eq!(res.events[1].attributes, [("_contract_addr", "contract0"), ("action", "execute_transaction"), ("tx_id", "1")]);
    
    let balance = app.wrap().query_balance(CARL.to_string(), DENOM).unwrap();
    assert_eq!(balance.amount, Uint128::new(1000));
    assert_eq!(balance.denom, DENOM);

    let resp: TxExecutionsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TxExecutions {})
        .unwrap();
    assert_eq!(
        resp.tx_executions.len(),
        1
    );
}

#[test]
fn exec_sign_transaction() {
    let mut app = mock_app();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![Addr::unchecked("owner").to_string(), ALICE.to_string()],
                threshold: 2,
                whitelist_enabled: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let _ = app.send_tokens(Addr::unchecked("owner"), addr.clone(), &[coin(10000, DENOM)]);
    let balance = app.wrap().query_balance(addr.clone(), DENOM).unwrap();
    assert_eq!(balance.amount, Uint128::new(10000));
    assert_eq!(balance.denom, DENOM);
    
    let messages = vec![
        BankMsg::Send {
            to_address: CARL.to_string(),
            amount: vec![coin(1000, DENOM)],
        }
        .into(),
    ];
    let msg: ExecuteMsg<Empty> = ExecuteMsg::ExecuteTransaction {
        msgs: messages,
    };
    let res = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    assert_eq!(res.events[1].attributes, [("_contract_addr", "contract0"), ("action", "execute_transaction"), ("tx_id", "1")]);
    
    let resp: TxExecutionsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TxExecutions {})
        .unwrap();
    assert_eq!(
        resp.tx_executions[0].status,
        Some(TxStatus::Pending),
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SignTransaction {
        tx_id: 1,
    };
    let res = app
        .execute_contract(
            Addr::unchecked(ALICE.to_string()),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    assert_eq!(res.events[1].attributes, [("_contract_addr", "contract0"), ("action", "sign_transaction"), ("tx_id", "1")]);

    let balance = app.wrap().query_balance(CARL.to_string(), DENOM).unwrap();
    assert_eq!(balance.amount, Uint128::new(1000));
    assert_eq!(balance.denom, DENOM);

    let resp: TxExecutionsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TxExecutions {})
        .unwrap();
    assert_eq!(
        resp.tx_executions[0].status,
        Some(TxStatus::Done),
    );
}

#[test]
fn exec_set_whitelist_addresses() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string()],
                threshold: 1,
                whitelist_enabled: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    assert_eq!(
        resp,
        WhitelistAddressesResponse {
            whitelist_addresses: vec![],
        }
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetWhitelistAddresses { 
        addresses: vec![ALICE.to_string(), BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    let resp_addresses: HashSet<String> = resp.whitelist_addresses.into_iter().collect();
    let expected_addresses: HashSet<String> = vec![ALICE.to_string(), BOB.to_string()].into_iter().collect();
    assert_eq!(resp_addresses, expected_addresses);

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetWhitelistAddresses { 
        addresses: vec![BOB.to_string(), CARL.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    let resp_addresses: HashSet<String> = resp.whitelist_addresses.into_iter().collect();
    let expected_addresses: HashSet<String> = vec![ALICE.to_string(), BOB.to_string(), CARL.to_string()].into_iter().collect();
    assert_eq!(resp_addresses, expected_addresses);
}

#[test]
fn exec_remove_whitelist_addresses() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string()],
                threshold: 1,
                whitelist_enabled: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    assert_eq!(
        resp,
        WhitelistAddressesResponse {
            whitelist_addresses: vec![],
        }
    );

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetWhitelistAddresses { 
        addresses: vec![ALICE.to_string(), BOB.to_string(), CARL.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    let resp_addresses: HashSet<String> = resp.whitelist_addresses.into_iter().collect();
    let expected_addresses: HashSet<String> = vec![ALICE.to_string(), BOB.to_string(), CARL.to_string()].into_iter().collect();
    assert_eq!(resp_addresses, expected_addresses);

    let msg: ExecuteMsg<Empty> = ExecuteMsg::RemoveWhitelistAddresses { 
        addresses: vec![BOB.to_string(), CARL.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: WhitelistAddressesResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::WhitelistAddresses {})
        .unwrap();
    assert_eq!(resp.whitelist_addresses, vec![ALICE.to_string()]);
}

#[test]
fn exec_set_transfer_limits() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string()],
                threshold: 1,
                whitelist_enabled: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: TransferLimitsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TransferLimits {})
        .unwrap();
    assert_eq!(
        resp,
        TransferLimitsResponse {
            transfer_limits: vec![],
        }
    );

    let coins = vec![
        Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "token2".to_string(),
            amount: Uint128::new(200),
        },
    ];

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetTransferLimits { 
        coins: coins.clone(),
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: TransferLimitsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TransferLimits {})
        .unwrap();
    
    let mut resp_coins = resp.transfer_limits;
    resp_coins.sort_by(|a, b| a.denom.cmp(&b.denom));
    assert_eq!(resp_coins, coins);

    let new_coins = vec![
        Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(150),
        },
        Coin {
            denom: "token3".to_string(),
            amount: Uint128::new(300),
        },
    ];

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetTransferLimits { 
        coins: new_coins.clone(),
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: TransferLimitsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TransferLimits {})
        .unwrap();
    
    let mut expected_coins = vec![
        Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(150),
        },
        Coin {
            denom: "token2".to_string(),
            amount: Uint128::new(200),
        },
        Coin {
            denom: "token3".to_string(),
            amount: Uint128::new(300),
        },
    ];
    expected_coins.sort_by(|a, b| a.denom.cmp(&b.denom));

    let mut resp_coins = resp.transfer_limits;
    resp_coins.sort_by(|a, b| a.denom.cmp(&b.denom));

    assert_eq!(resp_coins, expected_coins);
}

#[test]
fn exec_remove_transfer_limits() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admin: Addr::unchecked("owner").to_string(),
                signers: vec![ALICE.to_string()],
                threshold: 1,
                whitelist_enabled: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let coins = vec![
        Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "token2".to_string(),
            amount: Uint128::new(200),
        },
        Coin {
            denom: "token3".to_string(),
            amount: Uint128::new(300),
        },
    ];

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SetTransferLimits { 
        coins: coins.clone(),
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: TransferLimitsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TransferLimits {})
        .unwrap();
    let mut resp_coins = resp.transfer_limits;
    resp_coins.sort_by(|a, b| a.denom.cmp(&b.denom));
    assert_eq!(resp_coins, coins);

    let denoms_to_remove = vec!["token2".to_string(), "token3".to_string()];

    let msg: ExecuteMsg<Empty> = ExecuteMsg::RemoveTransferLimits { 
        denoms: denoms_to_remove.clone(),
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: TransferLimitsResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::TransferLimits {})
        .unwrap();
    
    let mut expected_coins = vec![
        Coin {
            denom: "token1".to_string(),
            amount: Uint128::new(100),
        },
    ];
    expected_coins.sort_by(|a, b| a.denom.cmp(&b.denom));

    let mut resp_coins = resp.transfer_limits;
    resp_coins.sort_by(|a, b| a.denom.cmp(&b.denom));

    assert_eq!(resp_coins, expected_coins);
}
