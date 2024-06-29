use cosmwasm_std::{entry_point, StdResult, Response, DepsMut, Env, MessageInfo, Api, Addr, Deps, Binary, Empty, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

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
        admins: map_validate(deps.api, &msg.admins)?,
        members: map_validate(deps.api, &msg.members)?,
        mutable: msg.mutable,
    };
    STATE.save(deps.storage, &cfg)?;
    Ok(Response::default())
}

pub fn map_validate(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    addresses.iter().map(|addr| api.addr_validate(addr)).collect()
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
        ExecuteMsg::AddAdmins { admins } => exec::add_admins(deps, info, admins),
        ExecuteMsg::RemoveAdmins { admins } => exec::remove_admins(deps, info, admins),
        ExecuteMsg::AddMembers { members } => exec::add_members(deps, info, members),
        ExecuteMsg::RemoveMembers { members } => exec::remove_members(deps, info, members),
        ExecuteMsg::SpendBalances { recipient, amount } =>  exec::spend_balances(deps, info, recipient, amount),
    }
}

mod exec {
    use cosmwasm_std::{Coin, BankMsg};

    use super::*;

    pub fn add_admins(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_state = STATE.load(deps.storage)?;
        if !curr_state.can_modify(info.sender.as_ref()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        
        let mut admins = map_validate(deps.api, &admins)?;
        curr_state.admins.append(&mut admins);
        STATE.save(deps.storage, &curr_state)?;

        Ok(Response::new().add_attribute("action", "add_admins"))
    }

    pub fn remove_admins (
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_state = STATE.load(deps.storage)?;
        if !curr_state.can_modify(info.sender.as_ref()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        let admins = map_validate(deps.api, &admins)?;
        curr_state.admins.retain(|curr_admin| !admins.contains(curr_admin));
        STATE.save(deps.storage, &curr_state)?;

        Ok(Response::new().add_attribute("action", "remove_admins"))
    }

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        members: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_state = STATE.load(deps.storage)?;
        if !curr_state.can_modify(info.sender.as_ref()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        
        let mut members = map_validate(deps.api, &members)?;
        curr_state.members.append(&mut members);
        STATE.save(deps.storage, &curr_state)?;

        Ok(Response::new().add_attribute("action", "add_members"))
    }

    pub fn remove_members (
        deps: DepsMut,
        info: MessageInfo,
        members: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_state = STATE.load(deps.storage)?;
        if !curr_state.can_modify(info.sender.as_ref()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        let members = map_validate(deps.api, &members)?;
        curr_state.members.retain(|curr_member| !members.contains(curr_member));
        STATE.save(deps.storage, &curr_state)?;

        Ok(Response::new().add_attribute("action", "remove_members"))
    }

    pub fn spend_balances (
        deps: DepsMut,
        info: MessageInfo,
        recipient: String,
        amount: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        let curr_state = STATE.load(deps.storage)?;
        if !curr_state.can_spend(info.sender.as_ref()) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let recipient_addr = deps.api.addr_validate(&recipient)?;
        let msg = BankMsg::Send { to_address: recipient_addr.to_string(), amount };
        
        let res = Response::new()
            .add_attribute("action", "spend_balances")
            .add_attribute("recipient", recipient)
            .add_message(msg);
        Ok(res)
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
        AdminList {} => to_json_binary(&query::admin_list(deps)?),
        Memberlist {} => to_json_binary(&query::member_list(deps)?),
    }
}

mod query {
    use crate::msg::{AdminListResponse, MemberListResponse};

    use super::*;

    pub fn admin_list(deps: Deps) -> StdResult<AdminListResponse> {
        let cfg = STATE.load(deps.storage)?;
        let resp = AdminListResponse{
            admins: cfg.admins.into_iter().map(|a| a.into()).collect(),
        };
        Ok(resp)
    }

    pub fn member_list(deps: Deps) -> StdResult<MemberListResponse> {
        let cfg = STATE.load(deps.storage)?;
        let resp = MemberListResponse{
            members: cfg.members.into_iter().map(|a| a.into()).collect(),
        };
        Ok(resp)
    }
}