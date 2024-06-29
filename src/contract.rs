use cosmwasm_std::{entry_point, StdResult, Response, DepsMut, Env, MessageInfo, Deps, Binary, Empty, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::{map_validate, validate_addr};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::execute::{change_admin, add_signers, remove_signers, spend_balances};
use crate::query::{admin, signer_list};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-account-management";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = State {
        admin: validate_addr(deps.api, &msg.admin)?,
        signers: map_validate(deps.api, &msg.signers)?,
        mutable: msg.mutable,
    };
    STATE.save(deps.storage, &cfg)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        ExecuteMsg::Freeze {  } => Ok(Response::new()),
        ExecuteMsg::ChangeAdmin { new_admin } => change_admin(deps, info, new_admin),
        ExecuteMsg::AddSigners { signers } => add_signers(deps, info, signers),
        ExecuteMsg::RemoveSigners { signers } => remove_signers(deps, info, signers),
        ExecuteMsg::SpendBalances { recipient, amount } => spend_balances(deps, info, recipient, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Admin {} => to_json_binary(&admin(deps)?),
        Signerlist {} => to_json_binary(&signer_list(deps)?),
    }
}