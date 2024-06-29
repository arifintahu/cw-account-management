use cosmwasm_std::{StdResult, Deps};
use crate::msg::{AdminListResponse, MemberListResponse};
use crate::state::STATE;

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