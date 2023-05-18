use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Entry, Pool, TokenInfo, AnnualInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    NewEntry {user: Addr, pool_id: u64, lp_token_amount: Decimal, token_1_amount: Decimal, token_2_amount: Decimal, pool_addr: String},
    UpdateEntry {user: Addr, pool_id: u64, lp_token_amount: Decimal, token_1_amount: Decimal, token_2_amount: Decimal},
    DeleteEntry { user: Addr, pool_id: u64},
    NewPool {pool_id: u64, token_1: TokenInfo, token_2: TokenInfo, apr: AnnualInfo, apy: AnnualInfo, tvl: Decimal, converted_tvl: Decimal, reward_coin: Vec<Addr>},
    UpdatePool {pool_id: u64, apr: AnnualInfo, apy: AnnualInfo, tvl: Decimal, converted_tvl: Decimal},
    RemovePool {pool_id: u64}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryAllPools {},
    QueryUserEntries {user: Addr},
    QueryAllEntries {},
}

// A custom struct is defined for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponseEntry {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListResponsePool {
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SpecificPoolResponse {
    pub pool: Pool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllEntriesResponse {
    pub entries: Vec<(Addr, Vec<Entry>)>,
}