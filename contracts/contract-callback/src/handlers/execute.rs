use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response};
use crate::{msg::{ExecuteMsg, MsgRequestCallback}, state::STATE, ContractError};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps),
        ExecuteMsg::Reset { count } => reset(deps, info, count),
        ExecuteMsg::Register { job_id } => register(deps, info, env, job_id),
    }
}

pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("action", "reset"))
}

pub fn register(deps: DepsMut, info: MessageInfo, env: Env, job_id: u64) -> Result<Response, ContractError> {
    let contract_address = env.contract.address.to_string();

    let callback_height = env.block.height + 5;
    
    let funds = &info.funds[0];

    let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: funds.denom.to_string(),
        amount: funds.amount.to_string()
    };
    let regsiter_msg = MsgRequestCallback {
        sender: contract_address.clone(),
        contract_address: contract_address.clone(),
        job_id: job_id.clone(),
        callback_height: callback_height.clone(),
        fees: Some(fee)
    };
    let register_stargate_msg = CosmosMsg::Stargate {
        type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
        value: Binary::from(prost::Message::encode_to_vec(&regsiter_msg)),
    };
    Ok(Response::new()
        .add_attribute("action", "register")
        .add_message(register_stargate_msg))
    }