use crate::{cw_serde, Config};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct ReplyCallbackInfo {
    pub callback_id: u32,
    pub receiver: Addr,
    pub channel_id: String,
}

// TODO: send a 'finalize' msg to self to clear this before the next caller of executemsgs
pub const ACTIVE_REPLY_CALLBACKS: Map<u32, ReplyCallbackInfo> = Map::new("active_reply_callbacks");
