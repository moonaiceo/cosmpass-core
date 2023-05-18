use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

// TokenInfo is the model that unites common token specific info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub symbol: String,
    pub denom: String,
    pub icon_url: String,
}
// AnnualInfo can used as model for: APR, APY
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AnnualInfo {
    pub one_day: Decimal,
    pub one_week: Decimal,
    pub two_week: Decimal,
}

// state.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub pool_id: u64,
    pub lp_token_amount: Decimal,
    pub token_1_amount: Decimal,
    pub token_2_amount: Decimal,
    pub pool_addr: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Pool {
    pub pool_id: u64,
    pub token_1: TokenInfo,
    pub token_2: TokenInfo,
    pub apr: AnnualInfo, 
    pub apy: AnnualInfo,
    // ToDo
    // Make apy automatically calculated from given apr, to reduce what is need to be filled by user
    pub tvl: Decimal,
    pub converted_tvl: Decimal,
    pub reward_coin: Vec<Addr>
}

pub const POOLS: Map<u64, Pool> = Map::new("pools");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LIST: Map<&Addr, Vec<Entry>> = Map::new("list");