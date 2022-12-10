#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr,
};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ListResponse};
use crate::state::{Config, CONFIG, ENTRY_SEQ, LIST, Entry};


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
        ExecuteMsg::NewEntry {user, pool_id, amount} => execute_create_new_entry(deps, info, user, pool_id, amount),
        ExecuteMsg::UpdateEntry {user, pool_id, amount} => execute_update_entry(deps, info, user, pool_id, amount),
        ExecuteMsg::DeleteEntry {user, pool_id} => execute_delete_entry(deps, info, user, pool_id)
    }
}

pub fn execute_create_new_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: String, amount: i128) -> Result<Response, ContractError> {
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
    // The new entry is defined with the received `amount` and `pool_id` attributes.
    let new_entry = Entry {
        id,
        amount,
        pool_id,
    };

    // The function first checks if user is new, or already in storage 
    if LIST.has(deps.storage, &user){
        let mut user_v = LIST.load(deps.storage, &user)?;
        user_v.push(new_entry);
        LIST.save(deps.storage, &user, &user_v)?;
    } else{
        let user_v = vec![new_entry];
        LIST.save(deps.storage, &user, &user_v)?;
    }

    // The function finally saves the new entry to the `LIST` with the matching `pool_id` and returns a `Response`
    // with the relevant attributes. 
    Ok(Response::new().add_attribute("method", "execute_add_new_entry")
        .add_attribute("new_entry_id", id.to_string()))
}

pub fn execute_update_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: String, amount: i128) -> Result<Response, ContractError> {
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

    for i in 0..user_v.len(){
        if user_v[i].pool_id == pool_id {
            user_v[i].amount = amount;
        }
    }
    LIST.save(deps.storage, &user, &user_v)?;
    // The function saves the updated entry to the `LIST` with the matching `pool_id` and returns a `Response` 
    // with the relevant attributes.
    Ok(Response::new().add_attribute("method", "execute_update_entry")
                      .add_attribute("updated_pool_id", pool_id.to_string()))
}

pub fn execute_delete_entry(deps: DepsMut, info: MessageInfo, user: Addr, pool_id: String) -> Result<Response, ContractError> {
    // Before carrying on with the removal, the function checks if the message sender is 
    // the owner of the contract.
    let owner = CONFIG.load(deps.storage)?.owner;
    if info.sender != owner {
        // If not, it returns an error and the deletion fails to be performed.
        return Err(ContractError::Unauthorized {});
    }
    // The entry with the matching `pool_id` is removed from the `POOLS`.

    let mut user_v = LIST.load(deps.storage, &user)?;
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryList {user} => to_binary(&query_list(deps, user)?),
    }
}

fn query_list(deps: Deps, user: Addr) -> StdResult<ListResponse> {
    // The entries that belong to matching `user` address are loaded from the `LIST`.
    let user_v = LIST.load(deps.storage, &user)?;
    let result = ListResponse {
        entries: user_v,
    };
    Ok(result)
    // Example of output:    {"data":{"entries":[{"id":1,"pool_id":"102","amount":"30100"}, {"id":2,"pool_id":"100","amount":"1500"}]}}
}
