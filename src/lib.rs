pub mod contract;
mod error;
pub mod state;

pub use crate::error::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub msgs: Option<Vec<CosmosMsg>>,
}

#[cw_serde]
pub struct ExecuteMsg {
    pub msgs: Vec<CosmosMsg>,
}

#[cw_serde]
pub struct ExecuteWasmMsg {}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
