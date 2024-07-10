use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, CosmosMsg};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub signers: Vec<Addr>,
    pub threshold: u8,
    pub mutable: bool,
}

impl State {
    // return true if the address is registered as admin
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admin.as_ref() == addr
    }

    // return true if the address is registered as signer
    pub fn is_signer(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.signers.iter().any(|a| a.as_ref() == addr)
    }

    // return true if the address is registered as admin and the config is mutable
    pub fn can_modify(&self, addr: &str) -> bool {
        self.mutable && self.is_admin(addr)
    }

    // return true if the address is registered as signer
    pub fn can_spend(&self, addr: &str) -> bool {
        self.is_signer(addr)
    }

    // return true if the address is registered as signer
    pub fn can_execute(&self, addr: &str) -> bool {
        self.is_signer(addr)
    }
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TxStatus {
    Pending,
    Done,
    Failed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TxData {
    pub id: u16,
    pub msgs: Vec<CosmosMsg>,
    pub signers: Vec<Addr>,
    pub status: Option<TxStatus>,
}

impl TxData {
    pub fn new(
        id: u16,
        msgs: Vec<CosmosMsg>,
        signer: Addr,
        status: TxStatus,
    ) -> Self {
        TxData{
            id,
            msgs,
            signers: vec![signer],
            status: Some(status),
        }
    }
}

pub const TX_NEXT_ID: Item<u16> = Item::new("tx_next_id");
pub const TX_EXECUTION: Map<u16, TxData> = Map::new("tx_execution");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Policy {
    pub whitelist_enabled: bool,
    pub whitelist_addresses: Vec<Addr>,
    pub transfer_limits: Vec<Coin>,
}

pub const POLICY: Item<Policy> = Item::new("policy");