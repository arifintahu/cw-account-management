use std::fmt;

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Coin, CosmosMsg, Empty};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::TxData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub signers: Vec<String>,
    pub threshold: u8,
    pub mutable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T = Empty> 
where
    T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    // Freeze will make a mutable contact immutable, must be called by an admin
    Freeze {},
    // ChangeAdmin will change current admin to new admin, must be called by a current admin
    ChangeAdmin { new_admin: String },
    // ChangeThreshold will change current threshold to new threshold, must be called by a current admin
    ChangeThreshold { new_threshold: u8 },
    // AddSigners will add signers to current signers, must be called by an admin
    AddSigners { signers: Vec<String> },
    // RemoveSigners will remove signers from current signers, must be called by an admin
    RemoveSigners { signers: Vec<String> },
    // SpendBalance will send token from smarcontract balance to recipient address
    SpendBalances { recipient: String, amount: Vec<Coin> },
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    ExecuteTransaction{ msgs: Vec<CosmosMsg<T>> },
    // SignMessage will sign transaction execution in pending period
    SignTransaction { tx_id: u16 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(AdminResponse)]
    Admin {},

    #[returns(SignerListResponse)]
    Signerlist {},

    #[returns(ThresholdResponse)]
    Threshold {},

    #[returns(ThresholdResponse)]
    TxExecutions {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AdminResponse {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignerListResponse {
    pub signers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ThresholdResponse {
    pub threshold: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TxExecutionsResponse {
    pub tx_executions: Vec<TxData>,
}