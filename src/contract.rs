use crate::state::CONFIG;
use crate::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::{ContractError, ExecuteMsgsMsg, UpdateOwnerMsg, UpdateWhitelistMsg};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};


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
    }
}

pub fn execute_msgs(
    deps: DepsMut,
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

    Ok(Response::new().add_messages(msg.msgs))
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary("")
}

pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
