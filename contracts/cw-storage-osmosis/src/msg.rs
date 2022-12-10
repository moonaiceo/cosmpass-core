use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Entry};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    NewEntry {user: Addr, pool_id: String, amount: i128},
    UpdateEntry {user: Addr, pool_id: String, amount: i128},
    DeleteEntry { user: Addr, pool_id: String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // QueryPool {user: Addr, pool_id: String},
    QueryList {user: Addr},
}

// A custom struct is defined for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub id: u64,
    pub pool_id: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpecificPoolResponse {
    pub entry: Entry,
}
