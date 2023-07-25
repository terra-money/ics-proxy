pub mod contract;
mod error;
pub mod state;

pub use crate::error::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg};

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
}

#[cw_serde]
pub struct ExecuteMsgsMsg {
    pub msgs: Vec<CosmosMsg>,
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
