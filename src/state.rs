use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub signers: Vec<Addr>,
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
}

pub const STATE: Item<State> = Item::new("state");