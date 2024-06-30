use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Threshold {threshold} is out of range")]
    InvalidThreshold { threshold: u8 },
}
