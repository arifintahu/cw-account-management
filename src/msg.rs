use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub members: Vec<String>,
    pub mutable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Freeze will make a mutable contact immutable, must be called by an admin
    Freeze {},
    // AddAdmins will add admins to current admins, must be called by an admin
    AddAdmins { admins: Vec<String> },
    // RemoveAdmins will remove admins from current admins, must be called by an admin
    RemoveAdmins { admins: Vec<String> },
    // AddMembers will add members to current members, must be called by an admin
    AddMembers { members: Vec<String> },
    // RemoveMembers will remove members from current members, must be called by an admin
    RemoveMembers { members: Vec<String> },
    // SpendBalance will send token from smarcontract balance to recipient address
    SpendBalances { recipient: String, amount: Vec<Coin> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(AdminListResponse)]
    AdminList {},

    #[returns(MemberListResponse)]
    Memberlist {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AdminListResponse {
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MemberListResponse {
    pub members: Vec<String>,
}