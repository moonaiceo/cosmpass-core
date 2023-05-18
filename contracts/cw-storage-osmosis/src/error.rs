use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized. Only owner of the contract can execute this message")]
    Unauthorized {},

    #[error("This entry already exists for this user")]
    UserEntryDuplicate { pool_id: String },

    #[error("No such entry for this user found")]
    EntryNotExists { pool_id: String },

    #[error("Pool is not found. (pool_id: {pool_id})")]
    PoolNotExists { pool_id: String },

    #[error("User does not exist in storage (user: {user})")]
    UserNotExists { user: String },

    #[error("User does not have any entries (user: {user})")]
    UserNoEntries { user: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
