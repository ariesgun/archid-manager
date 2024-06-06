use cosmwasm_std::{to_json_binary, Coin, CosmosMsg, DepsMut, Env, Response, SubMsg, Uint128, WasmMsg};

use crate::{msg::{ExecuteMsg, SudoMsg}, state::{RenewInfo, RENEW_MAP, STATE}, ContractError};
use std::u64;


pub fn sudo_handler(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
  match msg {
    SudoMsg::Callback { job_id } => handle_callback(deps, env, job_id),
    SudoMsg::Error { module_name, error_code, contract_address: _, input_payload, error_message } 
        => handle_error(deps, env, module_name, error_code, input_payload, error_message),
  }
}

pub fn handle_error(
    deps: DepsMut,
    env: Env,
    module_name: String,
    error_code: u32,
    input_payload: String,
    error_message: String,
) -> Result<Response, ContractError> {

    Ok(Response::new())
}

pub fn handle_callback(deps: DepsMut, env: Env, job_id: u64) -> Result<Response, ContractError> {
    // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //     if job_id == 0 {
    //     state.count -= 1;
    //     };
    //     if job_id >= 1 {
    //     state.count += 1;
    //     };
    //     Ok(state)
    // })?;

    // let contract_address = env.contract.address.to_string();
    // let callback_height = env.block.height + 5;

    // let fund: u128 = 22_005_000_000_000_000;

    // let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
    //     denom: "aconst".to_string(),
    //     amount: Uint128::from(fund).to_string(),
    // };
    // let regsiter_msg = MsgRequestCallback {
    //     sender: contract_address.clone(),
    //     contract_address: contract_address.clone(),
    //     job_id: job_id.clone(),
    //     callback_height: callback_height.clone(),
    //     fees: Some(fee)
    // };
    // let register_stargate_msg = CosmosMsg::Stargate {
    //     type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
    //     value: Binary::from(prost::Message::encode_to_vec(&regsiter_msg)),
    // };

    // let messages = vec![register_stargate_msg];

    // Renew domain
    // let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    // let cw721_contract = "archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008";

    // let renew_info = RENEW_MAP.load(deps.storage, job_id)?;
    // let domain_name = renew_info.domain_id;
    // let owner = renew_info.owner;
    // let owner = "archway1xtsm2ezhklnvmvw08y6ugjtmd6stdsqngkfmfn";
    // let domain_name = "testdomainx3".to_string();
    // let nft_id = domain_name.clone() + ".arch";
    // let denom = "aconst";
    // let cost_per_year: u128 = 250_000_000_000_000_000;

    // let transfer_nft_msg = archid_token::ExecuteMsg::TransferNft { 
    //     recipient: env.contract.address.to_string(),
    //     token_id: nft_id.clone() 
    // };

    // // Do registration
    // let transfer_execute: CosmosMsg = WasmMsg::Execute {
    //     contract_addr: cw721_contract.into(),
    //     msg: to_json_binary(&transfer_nft_msg)?,
    //     funds: vec![]
    // }.into();

    // let renew_domain_msg = archid_registry::msg::ExecuteMsg::RenewRegistration { 
    //     name: domain_name.to_string() 
    // };

    // let renew_execute: CosmosMsg = WasmMsg::Execute { 
    //     contract_addr: registry_contract.to_string(), 
    //     msg: to_json_binary(&renew_domain_msg)?,
    //     funds: vec![Coin {
    //         denom: denom.into(),
    //         amount: Uint128::from(cost_per_year), // E.g. register for 1 year
    //     }],
    // }
    // .into();

    // let transfer_nft_msg_2 = archid_token::ExecuteMsg::TransferNft { 
    //     recipient: owner.to_string(),
    //     token_id: nft_id.clone() 
    // };

    // // Do registration
    // let transfer_execute_2: CosmosMsg = WasmMsg::Execute {
    //     contract_addr: cw721_contract.into(),
    //     msg: to_json_binary(&transfer_nft_msg_2)?,
    //     funds: vec![]
    // }.into();

    // let messages = vec![transfer_execute];
    // let increment_msg = ExecuteMsg::Increment {};
    let mut renew_info = RENEW_MAP.load(deps.storage, job_id)?;
    renew_info.status = 111;
    
    let renew_msg = ExecuteMsg::RenewDomain { 
        domain_name: renew_info.domain_id
    };

    let increment_execute : CosmosMsg = WasmMsg::Execute {
        contract_addr: env.contract.address.into_string(),
        msg: to_json_binary(&renew_msg)?,
        funds: vec![Coin{
            denom: "aconst".to_string(), 
            amount: Uint128::from(250_000_000_000_000_000u128)
        }]
    }.into();

    let mut renew_info = RENEW_MAP.load(deps.storage, job_id)?;
    renew_info.status = 111;
    let _ = RENEW_MAP.save(deps.storage, job_id, &renew_info);

    // let messages = vec![increment_execute];

    let submessage1 = SubMsg::reply_on_success(increment_execute.clone(), job_id);
    // let submessage2 = SubMsg::reply_on_success(increment_execute, 999);
    let submessages = vec![submessage1];

    Ok(Response::new()
        .add_attribute("action", "handle_callback")
        .add_submessages(submessages)
        // .add_messages(messages)
    )
}