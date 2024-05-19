use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, Response, Uint128};

use crate::{msg::{MsgRequestCallback, SudoMsg}, state::STATE, ContractError};
use std::u64;


pub fn sudo_handler(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
  match msg {
    SudoMsg::Callback { job_id } => handle_callback(deps, env, job_id),
  }
}

pub fn handle_callback(deps: DepsMut, env: Env, job_id: u64) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if job_id == 0 {
        state.count -= 1;
        };
        if job_id == 1 {
        state.count += 1;
        };
        if job_id == 2 {
        return Err(ContractError::UnknownJobId {});
        }
        Ok(state)
    })?;

    // let cost_per_year: u128 = 15000000000000000;
    // let register_resp = BankMsg::Send {
    //     to_address: "archway1xtsm2ezhklnvmvw08y6ugjtmd6stdsqngkfmfn".to_string(),
    //     amount: vec![Coin {
    //         denom: "aconst".to_string(),
    //         amount: Uint128::from(cost_per_year),
    //     }]
    // };

    let contract_address = env.contract.address.to_string();
    let callback_height = env.block.height + 5;

    let fund: u128 = 22005000000000000;

    let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: "aconst".to_string(),
        amount: Uint128::from(fund).to_string(),
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

    let messages = vec![register_stargate_msg];

    Ok(Response::new()
        .add_attribute("action", "handle_callback")
        .add_messages(messages))
}