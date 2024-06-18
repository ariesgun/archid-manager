use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, Response, SubMsg, Uint128, WasmMsg};

use crate::{msg::{MsgRequestCallback, SudoMsg}, 
            state::{ACC_JOB_MAP, CONFIG, CUR_BLOCK_ID, RENEW_JOBS_MAP, RENEW_MAP, STATE},
            ContractError};
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
    _deps: DepsMut,
    _env: Env,
    _module_name: String,
    _error_code: u32,
    _input_payload: String,
    _error_message: String,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn handle_callback(deps: DepsMut, env: Env, job_id: u64) -> Result<Response, ContractError> {
    
    let cur_block_id = CUR_BLOCK_ID.load(deps.storage)?;
    
    if job_id == 0 {

        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
          state.count = cur_block_id;
          state.status = 1;
          state.callback_height = env.block.height;
          Ok(state)
        })?;

        let config = CONFIG.load(deps.storage)?;

        let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            denom: "aconst".to_string(),
            amount: Uint128::new(config.cron_fee_amount).to_string(),
        };    
        let regsiter_msg = MsgRequestCallback {
            sender: env.contract.address.to_string(),
            job_id: job_id.clone(),
            callback_height: env.block.height + u64::from(config.cron_period),
            contract_address: env.contract.address.to_string(),
            fees: Some(fee)
        };
        let register_stargate_msg = CosmosMsg::Stargate {
            type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
            value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&regsiter_msg)),
        };

        // Renew callback
        let fee2 = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            denom: "aconst".to_string(),
            amount: Uint128::new(150_000_000_000_000_000).to_string() // Gas fee
        };    
        let register_callback_msg = MsgRequestCallback {
            sender: env.contract.address.to_string(),
            job_id: cur_block_id.clone(),
            callback_height: env.block.height + 1,
            contract_address: env.contract.address.to_string(),
            fees: Some(fee2)
        };
        let register_renew_stargate_msg = CosmosMsg::Stargate {
            type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
            value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&register_callback_msg)),
        };

        let next_block_id = cur_block_id + 1;
        let _ = CUR_BLOCK_ID.save(deps.storage, &next_block_id);

        let submessage1 = SubMsg::reply_on_error(register_stargate_msg.clone(), 0);
        let submessage2 = SubMsg::reply_always(register_renew_stargate_msg.clone(), 1);
        let submessages = vec![submessage1, submessage2];

        Ok(Response::new()
            .add_attribute("action", "extend_cron_callback")
            .add_attribute("block_id", cur_block_id.to_string())
            .add_submessages(submessages)
        )
    } else {
        handle_renew_callback(deps, env, job_id)
    }
}

pub fn handle_renew_callback(deps: DepsMut, env: Env, job_id: u64) -> Result<Response, ContractError> {

    let block_id = job_id;
    let renew_jobs_at_idx = RENEW_JOBS_MAP.may_load(deps.storage, block_id)?;
    if renew_jobs_at_idx.is_some() && renew_jobs_at_idx.clone().unwrap().len() > 0 {

        let mut unwrapped_renew_jobs = renew_jobs_at_idx.unwrap();

        let domain_name = unwrapped_renew_jobs[0].to_string();
        let job_renew_id = ACC_JOB_MAP.load(deps.storage, domain_name.to_string())?;
        let mut renew_info = RENEW_MAP.load(deps.storage, job_renew_id)?;    

        let renew_excute : CosmosMsg = WasmMsg::Execute {
            contract_addr: env.contract.address.clone().into_string(),
            msg: renew_info.execute_msg.clone(),
            funds: vec![]
        }.into();

        unwrapped_renew_jobs.remove(0);
        let _ = RENEW_JOBS_MAP.save(deps.storage, block_id, &unwrapped_renew_jobs);

        let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            denom: "aconst".to_string(),
            amount: Uint128::new(150_000_000_000_000_000).to_string() // Gas fee
        };    
        let regsiter_msg = MsgRequestCallback {
            sender: env.contract.address.to_string(),
            job_id: block_id.clone(),
            callback_height: env.block.height + 1,
            contract_address: env.contract.address.to_string(),
            fees: Some(fee)
        };
        let register_stargate_msg = CosmosMsg::Stargate {
            type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
            value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&regsiter_msg)),
        };

        // let messages = vec![register_stargate_msg];
        
        renew_info.status = env.block.height;
        let _ = RENEW_MAP.save(deps.storage, job_renew_id, &renew_info);

        let submessage1 = SubMsg::reply_on_error(renew_excute.clone(), job_renew_id);
        let submessage2 = SubMsg::reply_always(register_stargate_msg.clone(), 1);

        let submessages = vec![submessage1, submessage2];

        Ok(Response::new()
            .add_attribute("action", "renew_callback")
            .add_attribute("job_id", job_renew_id.to_string())
            .add_attribute("domain", domain_name)
            .add_submessages(submessages)
            // .add_messages(messages)
        )
    } else {
        Ok(Response::new())
    }
}
