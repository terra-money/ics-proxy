use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub allow_cross_chain_msgs: bool,
    pub owner: Option<CanonicalAddr>,
    pub whitelist: Option<Vec<CanonicalAddr>>,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct ReplyCallbackInfo {
    pub callback_id: u32,
    pub receiver: String,
    pub port_id: String,
    pub channel_id: String,
    // denom to send back, as IBC hooks won't work without a coin sent back
    // will generally be the local chain's IBC denom for uluna
    pub denom: String,
}

// TODO: send a 'finalize' msg to self to clear this before the next caller of executemsgs?
pub const ACTIVE_REPLY_CALLBACKS: Map<u32, ReplyCallbackInfo> = Map::new("active_reply_callbacks");
