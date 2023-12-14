use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub allow_cross_chain_msgs: bool,
    pub allow_any_msg: bool,
    pub owner: Option<Addr>,
    pub whitelist: Option<Vec<Addr>>,
    /// channel-id for transfer packets, local chain -> Terra (what we use for dest_channel on Terra)
    pub terra_chain_channel: String,
    /// Address prefix for the chain this proxy is deployed to
    pub local_chain_bech32_prefix: String,
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
