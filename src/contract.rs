use crate::state::{ReplyCallbackInfo, ACTIVE_REPLY_CALLBACKS, CONFIG};
use crate::ContractError::Std;
use crate::{
    Config, ExecuteMsg, ExecuteMsgHook, ExecuteMsgInfo, ExecuteMsgReplyCallbackMsg, InstantiateMsg,
    QueryMsg,
};
use crate::{ContractError, ExecuteMsgsMsg, UpdateOwnerMsg, UpdateWhitelistMsg};
use cosmwasm_std::CosmosMsg::Stargate;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, Event, IbcMsg, IbcTimeout,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg, Timestamp,
};
use prost::Message;
use CosmosMsg::Ibc;
use IbcMsg::SendPacket;

const EXECUTE_MSG_CALLBACK_REPLY_ID: u64 = 1 << 32;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let owner = match msg.owner {
        None => None,
        Some(owner) => Some(deps.api.addr_validate(&owner)?),
    };
    let whitelist = match msg.whitelist {
        None => None,
        Some(whitelist) => {
            let mut result = vec![];
            for account in whitelist {
                result.push(deps.api.addr_validate(&account)?)
            }
            if owner.is_some() {
                result.push(owner.clone().unwrap())
            }
            result.dedup();
            Some(result)
        }
    };

    CONFIG.save(
        deps.storage,
        &Config {
            owner: owner.clone(),
            whitelist: whitelist.clone(),
        },
    )?;

    Ok(Response::new()
        .add_messages(msg.msgs.unwrap_or(vec![]))
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address)
        .add_attribute(
            "owner",
            owner.unwrap_or(Addr::unchecked(Addr::unchecked("None"))),
        )
        .add_attribute("whitelist", serde_json_wasm::to_string(&whitelist)?))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ExecuteMsgs(data) => execute_msgs(deps, env, info, data),
        ExecuteMsg::UpdateWhitelist(data) => update_whitelist(deps, env, info, data),
        ExecuteMsg::UpdateOwner(data) => update_owner(deps, env, info, data),
        ExecuteMsg::ExecuteMsgCallback(data) => execute_msg_callback(deps, env, info, data),
    }
}

pub fn execute_msgs(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsgsMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    match config.whitelist {
        None => {}
        Some(whitelist) => {
            if !whitelist.contains(&info.sender) {
                return Err(ContractError::Unauthorized {});
            }
        }
    }

    let submsgs = msg
        .msgs
        .into_iter()
        .enumerate()
        // TODO: verify vector length is < u32.MAX, otherwise casting usize to u32 below will panic
        .map(|(index, msg)| map_msg_info_to_submsg(deps.branch(), &info, index as u32, msg))
        .collect::<Result<Vec<SubMsg>, ContractError>>()?;

    Ok(Response::new()
        .add_attribute("action", "execute_msgs")
        .add_submessages(submsgs))
}

fn map_msg_info_to_submsg(
    deps: DepsMut,
    info: &MessageInfo,
    index: u32,
    msg: ExecuteMsgInfo,
) -> Result<SubMsg, ContractError> {
    match msg.reply_callback {
        None => Ok(SubMsg::new(msg.msg)),
        Some(reply_callback) => {
            ACTIVE_REPLY_CALLBACKS.save(
                deps.storage,
                index,
                &ReplyCallbackInfo {
                    callback_id: reply_callback.callback_id,
                    receiver: info.sender.clone(),
                    channel_id: reply_callback.ibc_channel,
                },
            )?;

            // we use 64 bits for the reply, assigning the first 32 bits to know which type of reply
            // we're handling in this contract, and the last 32 bits to know which callback index
            // we're handling
            let reply_id = EXECUTE_MSG_CALLBACK_REPLY_ID << 32 | (index as u64);

            Ok(SubMsg::reply_on_success(msg.msg, reply_id))
        }
    }
}

pub fn update_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateWhitelistMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    match config.owner.clone() {
        None => return Err(ContractError::Unauthorized {}),
        Some(owner) => {
            if info.sender != owner {
                return Err(ContractError::Unauthorized {});
            }
        }
    }

    let updated_whitelist = match msg.whitelist {
        None => None,
        Some(whitelist) => {
            let mut result = vec![];
            for account in whitelist {
                result.push(deps.api.addr_validate(&account)?)
            }
            result.push(config.owner.clone().unwrap());
            result.dedup();
            Some(result)
        }
    };

    CONFIG.save(
        deps.storage,
        &Config {
            owner: config.owner,
            whitelist: updated_whitelist.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "update_whitelist")
        .add_attribute("whitelist", serde_json_wasm::to_string(&updated_whitelist)?))
}

pub fn update_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: UpdateOwnerMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    match config.owner.clone() {
        None => return Err(ContractError::Unauthorized {}),
        Some(owner) => {
            if info.sender != owner {
                return Err(ContractError::Unauthorized {});
            }
        }
    }

    let updated_owner = match msg.owner {
        None => None,
        Some(owner) => Some(deps.api.addr_validate(&owner)?),
    };

    CONFIG.save(
        deps.storage,
        &Config {
            owner: updated_owner.clone(),
            whitelist: config.whitelist,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "update_owner")
        .add_attribute("owner", serde_json_wasm::to_string(&updated_owner)?))
}

pub fn execute_msg_callback(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsgReplyCallbackMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "execute_msg_callback"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("")
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let contract_reply_id = msg.id >> 32;

    match contract_reply_id {
        EXECUTE_MSG_CALLBACK_REPLY_ID => {
            let callback_index = msg.id as u32; // TODO: this should just truncate the leading bits, right? test this

            let reply_callback = ACTIVE_REPLY_CALLBACKS.load(deps.storage, callback_index)?; // TODO: handle case when not found gracefully

            // TODO: ofc, don't force unwrap here, check for results properly
            let events = msg.result.unwrap().events;

            // TODO: do we even care about replies to local chain? if so, discern in 'execute_msgs' whether it's local or not, and write down in ReplyCallbackInfo to use here
            let callback_msg = SubMsg::new(Ibc(SendPacket {
                channel_id: reply_callback.channel_id.clone(),
                data: to_binary(&encode_callback_msg(
                    &env,
                    events,
                    reply_callback.receiver.to_string(),
                    reply_callback.channel_id,
                )?)?,
                timeout: IbcTimeout::with_timestamp(Timestamp::from_nanos(u64::MAX)), // TODO: no timeout, right?
            }));
            Ok(Response::new().add_submessage(callback_msg))
        }
        _ => Err(Std(StdError::generic_err(format!(
            "unknown reply id: {}",
            msg.id
        )))),
    }
}

// TODO: move somewhere else
#[derive(Clone, PartialEq, prost::Message)]
struct Coin {
    #[prost(string, tag = "1")]
    pub denom: String,

    #[prost(string, tag = "2")]
    pub amount: String,
}

#[derive(Clone, PartialEq, prost::Message)]
struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: String,

    #[prost(string, tag = "2")]
    pub source_channel: String,

    #[prost(message, tag = "3")]
    pub token: Option<Coin>,

    #[prost(string, tag = "4")]
    pub sender: String,

    #[prost(string, tag = "5")]
    pub receiver: String,

    #[prost(uint64, tag = "7")]
    pub timeout_timestamp: u64,

    #[prost(string, tag = "8")]
    pub memo: String,
}

fn encode_callback_msg(
    env: &Env,
    events: Vec<Event>,
    receiver: String,
    channel: String,
) -> Result<String, ContractError> {
    let callback_msg =
        ExecuteMsgHook::ExecuteMsgReplyCallback(ExecuteMsgReplyCallbackMsg { events });

    let memo = format!(
        "{{\"wasm\":{{\"contract\":\"{}\",\"msg\":{}}}}}",
        receiver,
        serde_json_wasm::to_string(&callback_msg)?
    );

    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: channel,
        token: None,
        sender: env.contract.address.to_string(),
        receiver,
        timeout_timestamp: u64::MAX, // TODO: again no timeout, right?
        memo,
    }
    .encode_to_vec();

    let msg = serde_json_wasm::to_string(&Stargate::<String> {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value: msg.into(),
    })?;

    Ok(msg)
}

pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
