#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    AllowanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SetStatusMsg, StatusResponse,
};
use crate::state::{Status, STATUS_LIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:spark-blacklist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetStatus { status } => execute_set_status(deps, info, status),
    }
}

pub fn execute_set_status(
    deps: DepsMut,
    info: MessageInfo,
    status: SetStatusMsg,
) -> Result<Response, ContractError> {
    let current_status = STATUS_LIST.may_load(deps.storage, &info.sender)?;
    if let Some(_) = current_status {
        match status.clone() {
            SetStatusMsg::AllowAll => STATUS_LIST.remove(deps.storage, &info.sender),
            SetStatusMsg::AllowSome(allowlist) => {
                match current_status {
                    Some(_) => {
                        STATUS_LIST.update(deps.storage, &info.sender, |status| match status {
                            Some(_) => Ok(Status::AllowSome(allowlist)),
                            None => Err(ContractError::Std(StdError::NotFound {
                                kind: "status".into(),
                            })),
                        })?;
                    }
                    None => STATUS_LIST.save(
                        deps.storage,
                        &info.sender,
                        &Status::AllowSome(allowlist),
                    )?,
                };
            }
            SetStatusMsg::AllowNone => {
                match current_status {
                    Some(_) => {
                        STATUS_LIST.update(deps.storage, &info.sender, |status| match status {
                            Some(_) => Ok(Status::AllowNone),
                            None => Err(ContractError::Std(StdError::NotFound {
                                kind: "status".into(),
                            })),
                        })?;
                    }
                    None => STATUS_LIST.save(deps.storage, &info.sender, &Status::AllowNone)?,
                };
            }
        }
    }

    let new_status = match status {
        SetStatusMsg::AllowAll => "AllowAll",
        SetStatusMsg::AllowSome(_) => "AllowSome",
        SetStatusMsg::AllowNone => "AllowNone",
    };

    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("new_status", new_status))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStatus { address } => to_binary(&query_get_status(deps, address)?),
        QueryMsg::GetAllowance { address, sender } => {
            to_binary(&query_get_allowance(deps, address, sender)?)
        }
    }
}

fn query_get_status(deps: Deps, address: Addr) -> StdResult<StatusResponse> {
    let status = STATUS_LIST.may_load(deps.storage, &address)?;
    Ok(StatusResponse { status })
}

// Query if an address is allowed to donate on behalf of another address
fn query_get_allowance(deps: Deps, address: Addr, sender: Addr) -> StdResult<AllowanceResponse> {
    let status = STATUS_LIST.may_load(deps.storage, &address)?;
    match status {
        Some(status) => match status {
            Status::AllowNone => Ok(AllowanceResponse { allowance: false }),
            Status::AllowSome(allowlist) => Ok(AllowanceResponse {
                allowance: allowlist.contains(&sender),
            }),
        },
        None => Ok(AllowanceResponse { allowance: true }),
    }
}
