use cosmwasm_std::{Deps, Order, StdResult};
use crate::msg::{
    AdminResponse, SignerListResponse, ThresholdResponse, TransferLimitsResponse, TxExecutionsResponse, WhitelistAddressesResponse
};
use crate::state::{TxData, POLICY, STATE, TX_EXECUTION, TX_NEXT_ID};

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

pub fn tx_executions(deps: Deps) -> StdResult<TxExecutionsResponse> {
    let next_id = TX_NEXT_ID.load(deps.storage)?;
    if next_id <= 1 {
        let resp = TxExecutionsResponse{
            tx_executions: vec![],
        };
        Ok(resp)
    } else {
        let data: Vec<TxData> = TX_EXECUTION.range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        ).filter_map(|result| match result {
            Ok((_, d)) => Some(d.into()),
            Err(_) => None,
        }).collect();
        let resp = TxExecutionsResponse{
            tx_executions: data,
        };
        Ok(resp)
    }
}

pub fn whitelist_addresses(deps: Deps) -> StdResult<WhitelistAddressesResponse> {
    let policy = POLICY.load(deps.storage)?;
    let resp = WhitelistAddressesResponse{
        whitelist_addresses: policy.whitelist_addresses.into_iter().map(|a| a.into()).collect(),
    };
    Ok(resp)
}

pub fn transfer_limits(deps: Deps) -> StdResult<TransferLimitsResponse> {
    let policy = POLICY.load(deps.storage)?;
    let resp = TransferLimitsResponse{
        transfer_limits: policy.transfer_limits.into_iter().map(|a| a.into()).collect(),
    };
    Ok(resp)
}