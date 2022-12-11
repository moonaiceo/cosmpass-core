use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult, Uint128, WasmMsg};

/// Cw20ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
#[cw_serde]

pub struct Pair {
    pub denom: String,
    pub amount: Uint128,
}

impl Pair{
    pub fn help(self) -> String{
        return String::from("new");
    }
}
#[cw_serde]

pub struct AddLiquidityReceiveMsg {
    pub msg_type: String,
    pub sender: String,
    pub pool_id: String,
    pub share_out_amount: String,
    pub token_in_maxs: Vec<Pair>,
}

impl AddLiquidityReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverAddLiquidityExecuteMsg::Receive(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}


#[cw_serde]

pub struct AddBondReceiveMsg { 
    pub msg_type: String,
    pub owner: String,
    pub duration: String,
    pub coins: Vec<Pair>,
}


impl AddBondReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverBondExecuteMsg::Receive(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

#[cw_serde]

pub struct Cw20ReceiveMsg {
    pub sender: String,
    pub amount: Uint128,
    pub msg: Binary,
}

impl Cw20ReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::Receive(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

// This is just a helper to properly serialize the above message
#[cw_serde]

enum ReceiverExecuteMsg {
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]

enum ReceiverBondExecuteMsg{
    Receive(AddBondReceiveMsg),
}

#[cw_serde]

enum ReceiverAddLiquidityExecuteMsg{
    Receive(AddLiquidityReceiveMsg),
}