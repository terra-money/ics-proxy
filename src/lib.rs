pub mod contract;
mod error;
pub mod state;

#[cfg(test)]
mod tests;

pub use crate::error::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg, Event};

#[cw_serde]
pub struct Config {
    pub owner: Option<Addr>,
    pub whitelist: Option<Vec<Addr>>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub whitelist: Option<Vec<String>>,
    pub msgs: Option<Vec<CosmosMsg>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    ExecuteMsgs(ExecuteMsgsMsg),
    UpdateWhitelist(UpdateWhitelistMsg),
    UpdateOwner(UpdateOwnerMsg),

    ExecuteMsgCallback(ExecuteMsgReplyCallbackMsg),
}

#[cw_serde]
pub struct ExecuteMsgsMsg {
    pub msgs: Vec<ExecuteMsgInfo>,
}

#[cw_serde]
pub struct ExecuteMsgInfo {
    pub msg: CosmosMsg,
    pub reply_callback: Option<ReplyCallback>,
}

#[cw_serde]
pub struct ReplyCallback {
    pub callback_id: u32,
    pub ibc_channel: String,
}

#[cw_serde]
pub enum ExecuteMsgHook {
    ExecuteMsgReplyCallback(ExecuteMsgReplyCallbackMsg),
}

#[cw_serde]
pub struct ExecuteMsgReplyCallbackMsg {
    pub events: Vec<Event>,
}

#[cw_serde]
pub struct UpdateWhitelistMsg {
    pub whitelist: Option<Vec<String>>,
}

#[cw_serde]
pub struct UpdateOwnerMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub struct ExecuteWasmMsg {}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
