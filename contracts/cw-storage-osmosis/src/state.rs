use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");


//state.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub pool_id: String,
    pub amount: i128,
    // TODO
    // pub pool_addr: Addr/String
}

//TODO
// pub const POOLS: Map<String, u64> = Map::new("pools");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LIST: Map<&Addr, Vec<Entry>> = Map::new("list");