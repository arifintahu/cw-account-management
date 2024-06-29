use cosmwasm_std::{coin, Uint128, Addr};
use cw_multi_test::{App, ContractWrapper, Executor, AppBuilder};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, AdminListResponse, MemberListResponse};
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
fn query_admin_list() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![ALICE.to_string(), BOB.to_string()],
                members: vec![CARL.to_string()],
                mutable: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminListResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::AdminList {})
        .unwrap();
    assert_eq!(
        resp,
        AdminListResponse {
            admins: vec![ALICE.to_string(), BOB.to_string()],
        }
    )
}

#[test]
fn query_member_list() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![ALICE.to_string(), BOB.to_string()],
                members: vec![CARL.to_string()],
                mutable: false,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: MemberListResponse = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::Memberlist {})
        .unwrap();
    assert_eq!(
        resp,
        MemberListResponse {
            members: vec![CARL.to_string()],
        }
    )
}

#[test]
fn exec_add_admins() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![ALICE.to_string(), Addr::unchecked("owner").to_string()],
                members: vec![CARL.to_string()],
                mutable: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::AdminList {})
        .unwrap();
    assert_eq!(
        resp,
        AdminListResponse {
            admins: vec![ALICE.to_string(), Addr::unchecked("owner").to_string()],
        }
    );

    let msg = ExecuteMsg::AddAdmins { 
        admins: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: AdminListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::AdminList {})
        .unwrap();
    assert_eq!(
        resp,
        AdminListResponse {
            admins: vec![ALICE.to_string(), Addr::unchecked("owner").to_string(), BOB.to_string()],
        }
    );
}

#[test]
fn exec_remove_admins() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![ALICE.to_string(), BOB.to_string(), Addr::unchecked("owner").to_string()],
                members: vec![CARL.to_string()],
                mutable: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: AdminListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::AdminList {})
        .unwrap();
    assert_eq!(
        resp,
        AdminListResponse {
            admins: vec![ALICE.to_string(), BOB.to_string(), Addr::unchecked("owner").to_string()],
        }
    );

    let msg = ExecuteMsg::RemoveAdmins { 
        admins: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: AdminListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::AdminList {})
        .unwrap();
    assert_eq!(
        resp,
        AdminListResponse {
            admins: vec![ALICE.to_string(), Addr::unchecked("owner").to_string()],
        }
    );
}

#[test]
fn exec_add_members() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![Addr::unchecked("owner").to_string()],
                members: vec![ALICE.to_string()],
                mutable: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: MemberListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Memberlist {})
        .unwrap();
    assert_eq!(
        resp,
        MemberListResponse {
            members: vec![ALICE.to_string()],
        }
    );

    let msg = ExecuteMsg::AddMembers { 
        members: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: MemberListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Memberlist {})
        .unwrap();
    assert_eq!(
        resp,
        MemberListResponse {
            members: vec![ALICE.to_string(), BOB.to_string()],
        }
    );
}

#[test]
fn exec_remove_members() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                admins: vec![Addr::unchecked("owner").to_string()],
                members: vec![ALICE.to_string(), BOB.to_string()],
                mutable: true,
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let resp: MemberListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Memberlist {})
        .unwrap();
    assert_eq!(
        resp,
        MemberListResponse {
            members: vec![ALICE.to_string(), BOB.to_string()],
        }
    );

    let msg = ExecuteMsg::RemoveMembers { 
        members: vec![BOB.to_string()],
    };
    let _ = app
        .execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &msg,
            &[],
        ).unwrap();
    
    let resp: MemberListResponse = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::Memberlist {})
        .unwrap();
    assert_eq!(
        resp,
        MemberListResponse {
            members: vec![ALICE.to_string()],
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
                admins: vec![ALICE.to_string(), Addr::unchecked("owner").to_string()],
                members: vec![CARL.to_string()],
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

    let msg = ExecuteMsg::SpendBalances {
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