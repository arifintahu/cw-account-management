use cosmwasm_std::{coin, Addr, BankMsg, Empty, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor, AppBuilder};
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SignerListResponse, ThresholdResponse, TxExecutionsResponse};
use crate::contract::{instantiate, query, execute};

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
                mutable: false,
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
                mutable: false,
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
                mutable: true,
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
                mutable: true,
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
                mutable: true,
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
                mutable: true,
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
fn exec_spend_balances() {
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
                mutable: true,
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

    let msg: ExecuteMsg<Empty> = ExecuteMsg::SpendBalances {
        recipient: CARL.to_string(),
        amount: vec![coin(1000, DENOM)],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let balance = app.wrap().query_balance(CARL.to_string(), DENOM).unwrap();
    assert_eq!(balance.amount, Uint128::new(1000));
    assert_eq!(balance.denom, DENOM);
}

#[test]
fn exec_execute_messages() {
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
                mutable: true,
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
    let msg: ExecuteMsg<Empty> = ExecuteMsg::ExecuteMessages {
        msgs: messages,
    };
    let res = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    assert_eq!(res.events[1].attributes, [("_contract_addr", "contract0"), ("action", "execute_messages")]);
    
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