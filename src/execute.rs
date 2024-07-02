use std::fmt;

use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, MessageInfo, Response};
use schemars::JsonSchema;
use crate::error::ContractError;
use crate::state::STATE;
use crate::helpers::{is_valid_threshold, map_validate, validate_addr};

pub fn change_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    let mut curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    
    curr_state.admin = validate_addr(deps.api, &new_admin)?;
    STATE.save(deps.storage, &curr_state)?;

    Ok(
        Response::new()
            .add_attribute("action", "change_admin")
            .add_attribute("new_admin", new_admin)
    )
}

pub fn change_threshold(
    deps: DepsMut,
    info: MessageInfo,
    new_threshold: u8,
) -> Result<Response, ContractError> {
    let mut curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    if !is_valid_threshold(new_threshold, curr_state.signers.len()) {
        return Err(ContractError::InvalidThreshold {
            threshold: new_threshold,
        });
    }
    
    curr_state.threshold = new_threshold;
    STATE.save(deps.storage, &curr_state)?;

    Ok(
        Response::new()
            .add_attribute("action", "change_threshold")
            .add_attribute("new_threshold", new_threshold.to_string())
    )
}

pub fn add_signers(
    deps: DepsMut,
    info: MessageInfo,
    signers: Vec<String>,
) -> Result<Response, ContractError> {
    let mut curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    
    let mut signers = map_validate(deps.api, &signers)?;
    curr_state.signers.append(&mut signers);
    STATE.save(deps.storage, &curr_state)?;

    Ok(Response::new().add_attribute("action", "add_signers"))
}

pub fn remove_signers (
    deps: DepsMut,
    info: MessageInfo,
    signers: Vec<String>,
) -> Result<Response, ContractError> {
    let mut curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    let signers = map_validate(deps.api, &signers)?;
    curr_state.signers.retain(|curr_member| !signers.contains(curr_member));
    STATE.save(deps.storage, &curr_state)?;

    Ok(Response::new().add_attribute("action", "remove_signers"))
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

pub fn execute_messages<T>(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<CosmosMsg<T>>,
) -> Result<Response<T>, ContractError>
where
    T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let res = Response::new()
        .add_messages(msgs)
        .add_attribute("action", "execute_messages");
    Ok(res)
}