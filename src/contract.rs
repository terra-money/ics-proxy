use crate::ibc_hooks::{Coin, MsgTransfer};
use crate::state::{ReplyCallbackInfo, ACTIVE_REPLY_CALLBACKS, CONFIG};
use crate::ContractError::Std;
use crate::{
    Config, ExecuteMsg, ExecuteMsgHook, ExecuteMsgInfo, ExecuteMsgReplyCallbackMsg, InstantiateMsg,
    QueryMsg,
};
use crate::{ContractError, ExecuteMsgsMsg, UpdateOwnerMsg, UpdateWhitelistMsg};
use cosmwasm_std::CosmosMsg::Stargate;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg, SubMsgResult,
};
use prost::Message;
use ContractError::Unauthorized;
use ExecuteMsgHook::ExecuteMsgReplyCallback;

const EXECUTE_MSG_CALLBACK_REPLY_ID: u64 = 1;

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
        .add_attribute("owner", owner.unwrap_or(Addr::unchecked("None")))
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
                return Err(Unauthorized {});
            }
        }
    }

    // we cast index below to u32, so we need to verify they will be in supported range
    if msg.msgs.len() > u32::MAX as usize {
        return Err(StdError::generic_err(format!(
            "Messages array too long, must be shorter than {}",
            u32::MAX
        ))
        .into());
    }

    let submsgs = msg
        .msgs
        .into_iter()
        .enumerate()
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
            let receiver = match reply_callback.receiver {
                None => info.sender.to_string(),
                Some(receiver) => receiver,
            };
            ACTIVE_REPLY_CALLBACKS.save(
                deps.storage,
                index,
                &ReplyCallbackInfo {
                    callback_id: reply_callback.callback_id,
                    receiver,
                    channel_id: reply_callback.ibc_channel,
                    denom: reply_callback.denom,
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
        None => return Err(Unauthorized {}),
        Some(owner) => {
            if info.sender != owner {
                return Err(Unauthorized {});
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
        None => return Err(Unauthorized {}),
        Some(owner) => {
            if info.sender != owner {
                return Err(Unauthorized {});
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("")
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let contract_reply_id = msg.id >> 32;

    match contract_reply_id {
        EXECUTE_MSG_CALLBACK_REPLY_ID => {
            // truncate the leading 32 bits to get the callback index
            let callback_index = msg.id as u32;

            let reply_callback = ACTIVE_REPLY_CALLBACKS
                .may_load(deps.storage, callback_index)?
                .ok_or_else(|| {
                    StdError::generic_err(
                        "invalid state: reply callback info not found, but expected",
                    )
                })?;

            match msg.result {
                SubMsgResult::Ok(response) => {
                    // TODO: do we even care about replies to local chain? if so, how do we reliably discern if it's local or not?
                    let callback_msg = SubMsg::new(Stargate {
                        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
                        value: encode_callback_msg(
                            &env,
                            response.events,
                            response.data,
                            reply_callback.receiver,
                            reply_callback.channel_id,
                            reply_callback.denom,
                        )?,
                    });
                    Ok(Response::new().add_submessage(callback_msg))
                }
                SubMsgResult::Err(err) => Err(Std(StdError::generic_err(err))),
            }
        }
        _ => Err(Std(StdError::generic_err(format!(
            "unknown reply id: {}",
            msg.id
        )))),
    }
}

fn encode_callback_msg(
    env: &Env,
    events: Vec<Event>,
    data: Option<Binary>,
    receiver: String,
    channel: String,
    denom: String,
) -> Result<Binary, ContractError> {
    let callback_msg = ExecuteMsgReplyCallback(ExecuteMsgReplyCallbackMsg { events, data });

    let memo = format!(
        "{{\"wasm\":{{\"contract\":\"{}\",\"msg\":{}}}}}",
        receiver,
        serde_json_wasm::to_string(&callback_msg)?
    );

    let current_time = env.block.time;

    let msg = MsgTransfer {
        source_port: "transfer".to_string(),
        source_channel: channel,
        token: Some(Coin {
            denom,
            amount: "1".to_string(),
        }),
        sender: env.contract.address.to_string(),
        receiver,
        timeout_timestamp: current_time.plus_minutes(15).nanos(),
        memo,
    };

    Ok(msg.encode_to_vec().into())
}

pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
