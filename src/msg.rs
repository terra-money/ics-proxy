use crate::api::{ExecuteMsgReplyCallbackMsg, ExecuteMsgsMsg, UpdateOwnerMsg, UpdateWhitelistMsg};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;

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
pub enum ExecuteMsgHook {
    ExecuteMsgReplyCallback(ExecuteMsgReplyCallbackMsg),
}

#[cw_serde]
pub enum QueryMsg {}

#[cw_serde]
pub struct MigrateMsg {}
