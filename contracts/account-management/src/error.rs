use cosmwasm_std::{Addr, Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Threshold {threshold} is out of range")]
    InvalidThreshold { threshold: u8 },

    #[error("Status {tx_id} is not allowed")]
    InvalidStatus { tx_id: u16 },

    #[error("{recipient} is not whitelisted")]
    NotAllowedRecipient { recipient: String },

    #[error("{amount} is not allowed")]
    NotAllowedAmount { amount: Coin },
}
