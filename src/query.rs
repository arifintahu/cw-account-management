use cosmwasm_std::{StdResult, Deps};
use crate::msg::{AdminResponse, MemberListResponse};
use crate::state::STATE;

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let cfg = STATE.load(deps.storage)?;
    let resp = AdminResponse{
        admin: cfg.admin.to_owned().to_string(),
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