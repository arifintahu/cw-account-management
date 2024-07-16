use cosmwasm_std::{
    entry_point, StdResult, Response, DepsMut, Env, MessageInfo, Deps,
    Binary, Empty, to_json_binary,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::{map_validate, validate_addr, is_valid_threshold};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Policy, State, POLICY, STATE, TX_NEXT_ID};
use crate::execute::{
    add_signers, set_whitelist_addresses, change_admin, change_threshold,
    execute_transaction, remove_signers, remove_whitelist_addresses,
    sign_transaction,
};
use crate::query::{
    admin, signer_list, threshold, tx_executions, 
    whitelist_addresses,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-account-management";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INIT_TX_ID: u16 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<Empty>, ContractError> {
    if !is_valid_threshold(msg.threshold, msg.signers.len()) {
        return Err(ContractError::InvalidThreshold {
            threshold: msg.threshold,
        });
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = State {
        admin: validate_addr(deps.api, &msg.admin)?,
        signers: map_validate(deps.api, &msg.signers)?,
        threshold: msg.threshold,
    };
    STATE.save(deps.storage, &cfg)?;
    TX_NEXT_ID.save(deps.storage, &INIT_TX_ID)?;

    let policy = Policy {
        whitelist_enabled: msg.whitelist_enabled,
        whitelist_addresses: vec![],
        transfer_limits: vec![],
    };
    POLICY.save(deps.storage, &policy)?;
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
        ExecuteMsg::ChangeAdmin { new_admin } => change_admin(deps, info, new_admin),
        ExecuteMsg::ChangeThreshold { new_threshold } => change_threshold(deps, info, new_threshold),
        ExecuteMsg::AddSigners { signers } => add_signers(deps, info, signers),
        ExecuteMsg::RemoveSigners { signers } => remove_signers(deps, info, signers),
        ExecuteMsg::ExecuteTransaction { msgs } => execute_transaction(deps, info, msgs),
        ExecuteMsg::SignTransaction { tx_id } => sign_transaction(deps, info, tx_id),
        ExecuteMsg::SetWhitelistAddresses { addresses } => set_whitelist_addresses(deps, info, addresses),
        ExecuteMsg::RemoveWhitelistAddresses { addresses } => remove_whitelist_addresses(deps, info, addresses),
        ExecuteMsg::SetTransferLimits { coins } => Ok((Response::new())),
        ExecuteMsg::RemoveTransferLimits { denoms } => Ok((Response::new())),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {} => to_json_binary(&admin(deps)?),
        QueryMsg::Threshold {} => to_json_binary(&threshold(deps)?),
        QueryMsg::Signerlist {} => to_json_binary(&signer_list(deps)?),
        QueryMsg::TxExecutions {} => to_json_binary(&tx_executions(deps)?),
        QueryMsg::WhitelistAddresses {} => to_json_binary(&whitelist_addresses(deps)?),
    }
}