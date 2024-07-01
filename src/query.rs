use cosmwasm_std::{StdResult, Deps};
use crate::msg::{AdminResponse, SignerListResponse, ThresholdResponse};
use crate::state::STATE;

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let cfg = STATE.load(deps.storage)?;
    let resp = AdminResponse{
        admin: cfg.admin.to_owned().to_string(),
    };
    Ok(resp)
}

pub fn signer_list(deps: Deps) -> StdResult<SignerListResponse> {
    let cfg = STATE.load(deps.storage)?;
    let resp = SignerListResponse{
        signers: cfg.signers.into_iter().map(|a| a.into()).collect(),
    };
    Ok(resp)
}

pub fn threshold(deps: Deps) -> StdResult<ThresholdResponse> {
    let cfg = STATE.load(deps.storage)?;
    let resp = ThresholdResponse{
        threshold: cfg.threshold.to_owned(),
    };
    Ok(resp)
}