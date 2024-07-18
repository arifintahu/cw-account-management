use std::collections::{HashMap, HashSet};

use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, MessageInfo, Response, Uint128
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

pub fn change_whitelist_enabled(
    deps: DepsMut,
    info: MessageInfo,
    enabled: bool,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let mut curr_policy = POLICY.load(deps.storage)?;
    curr_policy.whitelist_enabled = enabled;
    POLICY.save(deps.storage, &curr_policy)?;

    Ok(
        Response::new()
            .add_attribute("action", "change_whitelist_enabled")
            .add_attribute("new_whitelist_enabled", enabled.to_string())
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
    
    let policy = POLICY.load(deps.storage)?;
    
    for send_msg in send_msgs {
        if let BankMsg::Send { to_address, amount } = send_msg {
            if !policy.can_receive(&to_address) {
                return Err(ContractError::NotAllowedRecipient {
                    recipient: to_address,
                });
            }

            for amt in amount {
                if !policy.can_transfer(amt.clone()) {
                    return Err(ContractError::NotAllowedAmount {
                        amount: amt,
                    });
                }                   
            }
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

pub fn set_whitelist_addresses(
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

pub fn set_transfer_limits(
    deps: DepsMut,
    info: MessageInfo,
    coins: Vec<Coin>,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    
    let mut curr_policy = POLICY.load(deps.storage)?;

    // Convert current transfer limits to a HashMap for easy updating
    let mut transfer_limits_map: HashMap<String, Uint128> = HashMap::new();
    for coin in curr_policy.transfer_limits {
        transfer_limits_map.insert(coin.denom.clone(), coin.amount);
    }
    
    // Update the transfer limits with the new coins
    for coin in coins.clone() {
        transfer_limits_map.insert(coin.denom.clone(), coin.amount);
    }
    
    // Convert the HashMap back to a Vec<Coin>
    curr_policy.transfer_limits = transfer_limits_map.into_iter()
        .map(|(denom, amount)| Coin { denom, amount })
        .collect();

    POLICY.save(deps.storage, &curr_policy)?;

    Ok(Response::new().add_attribute("action", "set_transfer_limits"))
}

pub fn remove_transfer_limits(
    deps: DepsMut,
    info: MessageInfo,
    denoms: Vec<String>,
) -> Result<Response, ContractError> {
    let curr_state = STATE.load(deps.storage)?;
    if !curr_state.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }
    
    let mut curr_policy = POLICY.load(deps.storage)?;
    
    // Convert the list of denominations to a HashSet for efficient lookup
    let denoms_set: HashSet<String> = denoms.into_iter().collect();
    
    // Remove coins with denominations in the denoms_set
    curr_policy.transfer_limits.retain(|coin| !denoms_set.contains(&coin.denom));
    
    POLICY.save(deps.storage, &curr_policy)?;
    
    Ok(Response::new().add_attribute("action", "remove_transfer_limits"))
}