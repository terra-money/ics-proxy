use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, CosmosMsg, Event};

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
    pub ibc_port: String,
    pub ibc_channel: String,
    // denom to send back when replying
    pub denom: String,
    pub receiver: Option<String>,
}

#[cw_serde]
pub struct ExecuteMsgReplyCallbackMsg {
    pub callback_id: u32,
    pub events: Vec<Event>,
    pub data: Option<Binary>,
}

#[cw_serde]
pub struct UpdateWhitelistMsg {
    pub whitelist: Option<Vec<String>>,
}

#[cw_serde]
pub struct UpdateOwnerMsg {
    pub owner: Option<String>,
}
