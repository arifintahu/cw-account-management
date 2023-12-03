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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        ExecuteMsg::Freeze {  } => Ok(Response::new()),
        ExecuteMsg::UpdateAdmins { admins } => Ok(Response::new()),
        ExecuteMsg::UpdateMembers { members } => Ok(Response::new()),
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