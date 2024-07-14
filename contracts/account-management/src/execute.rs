use std::collections::HashSet;

use cosmwasm_std::{
    Addr, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response
};
use crate::error::ContractError;
use crate::state::{
    TxData, TxStatus, POLICY, STATE, TX_EXECUTION, TX_NEXT_ID
};
use crate::helpers::{
    is_sufficient_signers, is_valid_threshold,
    map_validate, validate_addr,
};

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

pub fn execute_transaction(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<CosmosMsg>,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let send_msgs: Vec<BankMsg> = msgs.clone().into_iter()
        .filter_map(|msg| {
            if let CosmosMsg::Bank(bank_msg) = msg {
                if let BankMsg::Send { .. } = bank_msg {
                    Some(bank_msg)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    assert_eq!(send_msgs.len(), 1);

    for send_msg in send_msgs {
        if let BankMsg::Send { to_address, amount } = send_msg {
            println!("To Address: {}, Amount: {:?}", to_address, amount);
            // Do something with the amount
        }
    }

    if curr_state.threshold == 1 {
        let curr_id = TX_NEXT_ID.load(deps.storage).unwrap_or_default();
        let tx_data = TxData::new(curr_id, msgs.clone(), info.sender.clone(), TxStatus::Done);
        
        TX_EXECUTION.save(deps.storage, tx_data.id, &tx_data)?;
        TX_NEXT_ID.save(deps.storage, &(curr_id + 1))?;
        
        Ok(
            Response::new()
                .add_messages(msgs.clone())
                .add_attribute("action", "execute_transaction")
                .add_attribute("tx_id", curr_id.to_string())
        )
    } else if curr_state.threshold > 1 {
        let curr_id = TX_NEXT_ID.load(deps.storage).unwrap_or_default();
        let tx_data = TxData::new(curr_id, msgs.clone(), info.sender.clone(), TxStatus::Pending);
        
        TX_EXECUTION.save(deps.storage, tx_data.id, &tx_data)?;
        TX_NEXT_ID.save(deps.storage, &(curr_id + 1))?;

        Ok(
            Response::new()
                .add_attribute("action", "execute_transaction")
                .add_attribute("tx_id", curr_id.to_string())
        )
    } else {
        return Err(ContractError::InvalidThreshold {
            threshold: curr_state.threshold,
        });
    }
}

pub fn sign_transaction(
    deps: DepsMut,
    info: MessageInfo,
    tx_id: u16,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let mut tx = TX_EXECUTION.load(deps.storage, tx_id)?;
    if tx.status != Some(TxStatus::Pending) {
        return Err(ContractError::InvalidStatus {
            tx_id,
        });
    }

    tx.signers.push(info.sender.clone());

    let res = Response::new();
    if is_sufficient_signers(curr_state.threshold, tx.signers.len()) {
        tx.status = Some(TxStatus::Done);
        TX_EXECUTION.save(deps.storage, tx.id, &tx)?;
        Ok(
            res
                .add_messages(tx.msgs)
                .add_attribute("action", "sign_transaction")
                .add_attribute("tx_id", tx_id.to_string())
        )
    } else {
        TX_EXECUTION.save(deps.storage, tx.id, &tx)?;
        Ok(
            res
                .add_attribute("action", "sign_transaction")
                .add_attribute("tx_id", tx_id.to_string())
        )
    }
}

pub fn add_whitelist_addresses(
    deps: DepsMut,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    
    let addresses = map_validate(deps.api, &addresses)?;
    let mut curr_policy = POLICY.load(deps.storage)?;

    let mut unique_addresses: HashSet<Addr> = curr_policy.whitelist_addresses.into_iter().collect();
    unique_addresses.extend(addresses);
    
    curr_policy.whitelist_addresses = unique_addresses.into_iter().collect();
    POLICY.save(deps.storage, &curr_policy)?;

    Ok(Response::new().add_attribute("action", "add_whitelist_addresses"))
}

pub fn remove_whitelist_addresses (
    deps: DepsMut,
    info: MessageInfo,
    addresses: Vec<String>,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let addresses = map_validate(deps.api, &addresses)?;
    let mut curr_policy = POLICY.load(deps.storage)?;

    curr_policy.whitelist_addresses.retain(|curr_whitelist| !addresses.contains(curr_whitelist));
    POLICY.save(deps.storage, &curr_policy)?;

    Ok(Response::new().add_attribute("action", "remove_whitelist_addresses"))
}