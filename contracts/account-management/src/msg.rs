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
    pub whitelist_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T = Empty> 
where
    T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    // ChangeAdmin will change current admin to new admin, must be called by a current admin
    ChangeAdmin { new_admin: String },
    // ChangeThreshold will change current threshold to new threshold, must be called by a current admin
    ChangeThreshold { new_threshold: u8 },
    // AddSigners will add signers to current signers, must be called by an admin
    AddSigners { signers: Vec<String> },
    // RemoveSigners will remove signers from current signers, must be called by an admin
    RemoveSigners { signers: Vec<String> },
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    ExecuteTransaction{ msgs: Vec<CosmosMsg<T>> },
    // SignMessage will sign transaction execution in pending period
    SignTransaction { tx_id: u16 },
    // AddWhitelistAddresses will add whitelist addresses to account policy, must be called by an admin
    SetWhitelistAddresses { addresses: Vec<String> },
    // RemoveWhitelistAddresses will remove whitelist addresses from account policy, must be called by an admin
    RemoveWhitelistAddresses { addresses: Vec<String> },
    // AddWTransferLimits will add transfer limits to account policy, must be called by an admin
    SetTransferLimits { coins: Vec<Coin> },
    // RemoveTransferLimits will remove transfer limits from account policy, must be called by an admin
    RemoveTransferLimits { denoms: Vec<String> },
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

    #[returns(TxExecutionsResponse)]
    TxExecutions {},

    #[returns(WhitelistAddressesResponse)]
    WhitelistAddresses {},

    #[returns(TransferLimitsResponse)]
    TransferLimits {},
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistAddressesResponse {
    pub whitelist_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TransferLimitsResponse {
    pub transfer_limits: Vec<Coin>,
}