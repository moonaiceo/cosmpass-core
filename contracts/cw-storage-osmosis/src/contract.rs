#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Decimal, Order,
};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ListResponseEntry, ListResponsePool, AllEntriesResponse};
use crate::state::{Config, CONFIG, ENTRY_SEQ, LIST, Entry, Pool, POOLS, TokenInfo, AnnualInfo};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);
    // If the instantiation message contains an owner address, validate the address and use it.
    // Otherwise, the owner is the address that instantiates the contract.    

    let config = Config {
        owner: owner.clone()
    };
    // Save the owner address to contract storage.
    CONFIG.save(deps.storage, &config)?;
    // Save the entry sequence to contract storage, starting from 0.
    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewEntry {user, pool_id, lp_token_amount, token_1_amount, token_2_amount, pool_addr} => execute_create_new_entry(deps, info, user, pool_id, lp_token_amount, token_1_amount, token_2_amount, pool_addr),
        ExecuteMsg::UpdateEntry {user, pool_id, lp_token_amount, token_1_amount, token_2_amount} => execute_update_entry(deps, info, user, pool_id, lp_token_amount, token_1_amount, token_2_amount),
        ExecuteMsg::DeleteEntry {user, pool_id} => execute_delete_entry(deps, info, user, pool_id),
        ExecuteMsg::NewPool {pool_id, token_1, token_2, apr, apy, tvl, converted_tvl, reward_coin} => execute_create_new_pool(deps, info, pool_id,  token_1, token_2,apr, apy, tvl, converted_tvl, reward_coin),
        ExecuteMsg::UpdatePool {pool_id, apr, apy, tvl, converted_tvl} => execute_update_pool(deps, info, pool_id, apr, apy, tvl, converted_tvl),
        ExecuteMsg::RemovePool {pool_id} => execute_remove_pool(deps, info, pool_id)
    }
}

pub fn execute_create_new_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: u64, lp_token_amount: Decimal, token_1_amount: Decimal, token_2_amount: Decimal, pool_addr: String) -> Result<Response, ContractError> {
    // Before creating the new entry, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the new entry creation fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    // In order to generate a unique `id` for the new entry, the function increments the entry sequence 
    // and saves it to the contract storage with `ENTRY_SEQ.update()`.
    let id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| {
        Ok(id.add(1))
    })?;
    //The new entry is defined with the received `pool_id` and `amount` attributes. 

    let new_entry = Entry {
        id,
        lp_token_amount,
        token_1_amount,
        token_2_amount,
        pool_id,
        pool_addr,
    };

    // Before creating the new entry, the function checks if the user is in storage.
    if LIST.has(deps.storage, &user){
        let mut user_v = LIST.load(deps.storage, &user)?;
        if user_v.iter().find(|e| e.pool_id == new_entry.pool_id).is_none() {
            user_v.push(new_entry);
            LIST.save(deps.storage, &user, &user_v)?;
        } else {
            return Err(ContractError::UserEntryDuplicate { pool_id: pool_id.to_string() });
        }
    } else{
        let user_v = vec![new_entry];
        LIST.save(deps.storage, &user, &user_v)?;
    }

    // The function finally saves the new entry to the `LIST` with the matching `user` and returns a `Response`
    // with the relevant attributes. 
    Ok(Response::new().add_attribute("method", "execute_add_new_entry")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: u64, lp_token_amount: Decimal, token_1_amount: Decimal, token_2_amount: Decimal) -> Result<Response, ContractError> {
    // Before continuing with the new update, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the update fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    // The function is used to update amount of coins stored in specified pool
    // The entries that belong to the matching `user` are loaded from the `LIST`.
    let mut user_v = LIST.load(deps.storage, &user)?;
    if user_v.iter().find(|e| e.pool_id == pool_id).is_none() {
        return Err(ContractError::EntryNotExists { pool_id: pool_id.to_string() });
    }
    
    for i in 0..user_v.len(){
        if user_v[i].pool_id == pool_id {
            user_v[i].lp_token_amount = lp_token_amount;
            user_v[i].token_1_amount = token_1_amount;
            user_v[i].token_2_amount = token_2_amount;
            break
        }
    }  

    LIST.save(deps.storage, &user, &user_v)?;
    // The function saves the updated entry to the `LIST` with the matching `pool_id` and returns a `Response` 
    // with the relevant attributes.
    Ok(Response::new().add_attribute("method", "execute_update_entry")
                      .add_attribute("updated_pool_id", pool_id.to_string()))
}

pub fn execute_delete_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: u64) -> Result<Response, ContractError> {
    // Before carrying on with the removal, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the deletion fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    // The entry with the matching `pool_id` is removed from the `POOLS`.
    let mut user_v = LIST.load(deps.storage, &user)?;

    if user_v.iter().find(|e| e.pool_id == pool_id).is_none() {
        return Err(ContractError::EntryNotExists { pool_id: pool_id.to_string() });
    }

    for i in 0..user_v.len(){
        if user_v[i].pool_id == pool_id {
            user_v.remove(i);
            break;
        }
    }
    LIST.save(deps.storage, &user, &user_v)?;
    // The function returns a `Response` with the relevant attributes.
    Ok(Response::new().add_attribute("method", "execute_delete_entry")
                      .add_attribute("deleted_pool_id", pool_id.to_string()))
}

pub fn execute_create_new_pool(deps: DepsMut, info: MessageInfo, pool_id: u64, token_1: TokenInfo, token_2: TokenInfo, apr: AnnualInfo, apy: AnnualInfo, tvl: Decimal, converted_tvl: Decimal, reward_coin: Vec<Addr>) -> Result<Response, ContractError> {
    // Before creating the new pool, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the new entry creation fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    
    //The new pool is defined with the received attributes. 
    let new_pool = Pool {
        pool_id,
        token_1,
        token_2,
        apr, 
        apy,
        tvl,
        converted_tvl,
        reward_coin,
    };

    POOLS.save(deps.storage, pool_id, &new_pool)?;
    
    // The function finally saves the new entry to the `LIST` with the matching `user` and returns a `Response`
    // with the relevant attributes. 
    Ok(Response::new().add_attribute("method", "execute_add_new_pool")
        .add_attribute("new_pool_id", pool_id.to_string()))
}

pub fn execute_update_pool(deps: DepsMut, info: MessageInfo, pool_id: u64, apr: AnnualInfo, apy: AnnualInfo, tvl: Decimal, converted_tvl: Decimal) -> Result<Response, ContractError> {
    // Before continuing with the new update, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the update fails to be performed.
        return Err(ContractError::Unauthorized {});
    }

    if POOLS.has(deps.storage, pool_id) == false {
        return Err(ContractError::PoolNotExists { pool_id: pool_id.to_string() });
    }
    let pool = POOLS.load(deps.storage, pool_id)?;
    
    let updated_pool = Pool {
        pool_id,
        token_1: pool.token_1,
        token_2: pool.token_2,
        apr, 
        apy,
        tvl,
        converted_tvl,
        reward_coin: pool.reward_coin,
    };
    POOLS.save(deps.storage, pool_id, &updated_pool)?;
    // The function saves the updated entry to the `LIST` with the matching `user` and returns a `Response` 
    // with the relevant attributes.
    Ok(Response::new().add_attribute("method", "execute_update_pool")
                      .add_attribute("updated_pool_id", pool_id.to_string()))
}

pub fn execute_remove_pool(deps: DepsMut, info: MessageInfo, pool_id: u64) -> Result<Response, ContractError> {
    // Before carrying on with the removal, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the deletion fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    if POOLS.has(deps.storage, pool_id) == false {
        // If no such pool, it returns an error and the deletion fails to be performed.
        return Err(ContractError::PoolNotExists { pool_id: pool_id.to_string() });
    }
    // The entry with the matching `pool_id` is removed from the `POOLS`.
    POOLS.remove(deps.storage, pool_id);
    // The function returns a `Response` with the relevant attributes.
    Ok(Response::new().add_attribute("method", "execute_remove_pool")
                      .add_attribute("deleted_pool_id", pool_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryAllPools {} => to_binary(&query_all_pools(deps)?).map_err(ContractError::from),
        QueryMsg::QueryUserEntries {user} => to_binary(&query_user_entries(deps, user)?).map_err(ContractError::from),
        QueryMsg::QueryAllEntries {} => to_binary(&query_all_entries(deps)?).map_err(ContractError::from),
    }
}

fn query_user_entries(deps: Deps, user: Addr) -> Result<ListResponseEntry, ContractError> {
    // The entries with the matching `user` address are loaded from the `LIST`.
    if LIST.has(deps.storage, &user) == false {
        return Err(ContractError::UserNotExists { user: user.to_string() });
    }
    let user_v = LIST.load(deps.storage, &user)?;
    if user_v.is_empty() {
        return Err(ContractError::UserNoEntries { user: user.to_string() });
    }
    // An `ListResponseEntry` is formed with the attributes of the loaded entries and returned.
    let result = ListResponseEntry {
        entries: user_v,
    };
    Ok(result)
    // Example of output:    {"data":{"entries":[{"id":1,"pool_id":"102","amount":"30100"}, {"id":2,"pool_id":"100","amount":"1500"}]}}
}

fn query_all_entries(deps: Deps) -> StdResult<AllEntriesResponse> {
    let mut entries = vec![];
    let keys = LIST.keys(deps.storage, None, None, Order::Ascending);
    
    for key in keys {
        let addr: Addr = key.unwrap();
        let value = LIST.load(deps.storage, &addr)?;
        entries.push((addr, value));
    }

    // An `AllEnriesResponse` is formed with the attributes of the loaded entries that consist of keys and values.
    let result = AllEntriesResponse {
        entries,
    };
    Ok(result)
    // Example of output:    {"data":{"user_1":{"entries":[{"id":1,"pool_id":"102","amount":"30100"}, {"id":2,"pool_id":"100","amount":"1500"}]}}}
}

fn query_all_pools(deps: Deps) -> StdResult<ListResponsePool> {
    // All available pools from storage loaded.
    let mut pools = vec![];
    let keys = POOLS.keys(deps.storage, None, None, Order::Ascending);
    // An `ListResponsePool` is formed with the attributes of the loaded entry and returned.
    for key in keys {
        let pool_id: u64 = key.unwrap();
        let value = POOLS.load(deps.storage, pool_id)?;
        pools.push(value);
    }
    let result = ListResponsePool {
        pools,
    };
    Ok(result)
}
