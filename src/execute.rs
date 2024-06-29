use cosmwasm_std::{Response, DepsMut, MessageInfo, Coin, BankMsg};
use crate::error::ContractError;
use crate::state::STATE;
use crate::helpers::{map_validate, validate_addr};

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

    Ok(Response::new().add_attribute("action", "change_admin"))
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