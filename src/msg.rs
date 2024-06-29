use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub signers: Vec<String>,
    pub mutable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Freeze will make a mutable contact immutable, must be called by an admin
    Freeze {},
    // ChangeAdmin will change current admin to new admin, must be called by a current admin
    ChangeAdmin { new_admin: String },
    // AddSigners will add signers to current signers, must be called by an admin
    AddSigners { signers: Vec<String> },
    // RemoveSigners will remove signers from current signers, must be called by an admin
    RemoveSigners { signers: Vec<String> },
    // SpendBalance will send token from smarcontract balance to recipient address
    SpendBalances { recipient: String, amount: Vec<Coin> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(AdminResponse)]
    Admin {},

    #[returns(SignerListResponse)]
    Signerlist {},
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