use crate::api::{
    ConfigResponse, ExecuteMsgReplyCallbackMsg, ExecuteMsgsMsg, UpdateOwnerMsg, UpdateWhitelistMsg,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CosmosMsg;

#[cw_serde]
pub struct InstantiateMsg {
    /// This is a flag that can block this contract from executing cross-chain messages.
    /// Mainly used to prevent fake reports of this contract's callbacks.
    pub allow_cross_chain_msgs: bool,
    pub allow_any_msg: Option<bool>,
    /// channel-id for transfer packets, local chain -> Terra (what we use for dest_channel on Terra)
    pub terra_chain_channel: String,
    pub local_chain_bech32_prefix: String,
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
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}
