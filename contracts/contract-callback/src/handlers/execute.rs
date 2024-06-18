use cosmwasm_std::{coins, to_json_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128, WasmMsg};
use prost::Message;
use crate::{msg::{ExecuteMsg, MsgCancelCallback, MsgRegisterInterchainAccount, MsgRequestCallback, MsgSendTx, MsgVote}, state::{RenewInfo, ACC_JOB_MAP, CONFIG, CUR_BLOCK_ID, DEFAULT_ID, ICA_STATE, JOBS, RENEW_JOBS_MAP, RENEW_MAP, STATE}, ContractError};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps),
        ExecuteMsg::MintDomain { domain_name } => mint_domain(deps, info, env, domain_name),
        ExecuteMsg::RenewDomain { domain_name } => renew_domain(deps, info, env, domain_name),
        ExecuteMsg::ScheduleAutoRenew { domain_name } => schedule_auto_renew(deps, info, env, domain_name),
        ExecuteMsg::SetDefault { domain_name } => set_default(deps, info, env, domain_name),
        ExecuteMsg::CancelAutoRenew { domain_name } => cancel_auto_renew(deps, info, env, domain_name),
        ExecuteMsg::StartCronJob {} => start_cron_job_callback(deps, info, env),
        ExecuteMsg::StopCronJob {} => stop_cron_job_callback(deps, info, env),
        ExecuteMsg::Deposit {} => deposit_funds(deps, info, env),
        ExecuteMsg::Withdraw {} => withdraw(deps, info, env),
        ExecuteMsg::RegisterIca {connection_id} => register_ica(deps, info, env, connection_id),
        ExecuteMsg::Vote {
            proposal_id,
            option,
            connection_id,
            tiny_timeout,
        } => vote(deps.as_ref(), env, proposal_id, option, connection_id, tiny_timeout),
    }
}

pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

pub fn mint_domain(deps: DepsMut, info: MessageInfo, _env: Env, domain_name: String) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    
    let funds = &info.funds[0];

    // let res = cw_utils::must_pay(&info, &denom)?;
    // let registration: u64 =
    //     u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    // if registration < 1 {
    //     return Err(ContractError::InvalidPayment { amount: res });
    // }

    // Create registration msg
    let register_msg = archid_registry::msg::ExecuteMsg::Register {
        name: domain_name.clone().into(),
    };
    // Do registration
    let register_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.clone().into(),
        msg: to_json_binary(&register_msg)?,
        funds: vec![funds.clone()],
        }
    .into();

    let update_msg = archid_registry::msg::ExecuteMsg::UpdateResolver {
        name: domain_name.clone(),
        new_resolver: info.sender.clone(),
    };
    let update_resolver_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.into(),
        msg: to_json_binary(&update_msg)?,
        funds: vec![],
        }
    .into();

    let transfer_msg: archid_token::ExecuteMsg = archid_token::ExecuteMsg::TransferNft { 
        recipient: info.sender.to_string(),
        token_id: domain_name + ".arch",
    };

    let transfer_nft: CosmosMsg = WasmMsg::Execute { 
        contract_addr: cw721_contract.to_string(),
        msg: to_json_binary(&transfer_msg)?,
        funds: vec![] 
    }.into();

    let messages = vec![register_resp, update_resolver_msg, transfer_nft];

    Ok(Response::new()
        .add_attribute("action", "mint_domain")
        .add_messages(messages)
    )
}

pub fn set_default(deps: DepsMut, info: MessageInfo, _env: Env, domain_id: String) -> Result<Response, ContractError> {
    
    let config = CONFIG.load(deps.storage)?;
    let cw721_contract = config.cw721_archid_addr;
    
    // Check if it is owned by the sender
    let query_msg:  archid_token::QueryMsg<archid_token::Extension>  = archid_token::QueryMsg::OwnerOf { 
        token_id: domain_id.to_string(),
        include_expired: None
    };

    let res: cw721_updatable::OwnerOfResponse = deps.querier.query_wasm_smart(
        cw721_contract.to_string(),
        &query_msg
    )?;

    if res.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    let _ = DEFAULT_ID.save(deps.storage, info.sender.to_owned(), &domain_id);

    Ok(Response::new()
        .add_attribute("action", "set_default")
        .add_attribute("address", info.sender)
        .add_attribute("default", domain_id)
    )
}

pub fn renew_domain(deps: DepsMut, info: MessageInfo, env: Env, domain_name: String) -> Result<Response, ContractError> {
    
    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;

    let query_msg  = archid_registry::msg::QueryMsg::Config {};
    let config: archid_registry::state::Config = deps.querier.query_wasm_smart(
        registry_contract.to_string(), 
        &query_msg)?;
    let cost_per_year = config.base_cost;

    let nft_id = domain_name.clone() + ".arch";

    // let res: Uint128 = cw_utils::must_pay(&info, &String::from(denom.clone()))?;
    // let registration: u64 =
    //     u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    // if registration < 1 {
    //     return Err(ContractError::InvalidPayment { amount: res });
    // }

    let transfer_nft_msg = archid_token::ExecuteMsg::TransferNft { 
        recipient: env.contract.address.to_string(),
        token_id: nft_id.to_string() 
    };

    // Transfer to contract
    let transfer_execute: CosmosMsg = WasmMsg::Execute {
        contract_addr: cw721_contract.to_string(),
        msg: to_json_binary(&transfer_nft_msg)?,
        funds: vec![]
    }.into();

    let renew_domain_msg = archid_registry::msg::ExecuteMsg::RenewRegistration { 
        name: domain_name.to_string() 
    };

    let renew_execute: CosmosMsg = WasmMsg::Execute { 
        contract_addr: registry_contract.to_string(), 
        msg: to_json_binary(&renew_domain_msg)?,
        funds: vec![Coin {
            denom: denom.into(),
            amount: Uint128::from(cost_per_year), // E.g. register for 1 year
        }],
    }
    .into();

    let mut transfer_to = info.sender.clone();
    if info.sender == env.contract.address {
        let job_id = ACC_JOB_MAP.load(deps.storage, domain_name.clone())?;
        let renew_info = RENEW_MAP.load(deps.storage, job_id)?;
        transfer_to = renew_info.owner;
    }

    // Update resolver
    let update_msg = archid_registry::msg::ExecuteMsg::UpdateResolver {
        name: domain_name.clone(),
        new_resolver: transfer_to.clone(),
    };
    let update_resolver_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.into(),
        msg: to_json_binary(&update_msg)?,
        funds: vec![],
        }
    .into();

    let transfer_nft_msg_2 = archid_token::ExecuteMsg::TransferNft { 
        recipient: transfer_to.to_string(),
        token_id: nft_id.to_string() 
    };

    // Transfer back
    let transfer_execute_2: CosmosMsg = WasmMsg::Execute {
        contract_addr: cw721_contract.into(),
        msg: to_json_binary(&transfer_nft_msg_2)?,
        funds: vec![]
    }.into();

    let messages = vec![transfer_execute, renew_execute, update_resolver_msg, transfer_execute_2];

    Ok(Response::new()
        .add_attribute("action", "renew_domain")
        .add_attribute("domain", domain_name)
        .add_messages(messages)
    )
}

pub fn schedule_auto_renew(deps: DepsMut, info: MessageInfo, env: Env, domain_name: String) -> Result<Response, ContractError> 
{
    let config = CONFIG.load(deps.storage)?;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;
    
    let funds = &info.funds[0];
    let nft_id = domain_name.to_string() + ".arch";

    let state = STATE.load(deps.storage)?;

    let query_msg:  archid_token::QueryMsg<archid_token::Extension>  = archid_token::QueryMsg::Approval { 
        token_id: nft_id.to_string(),
        spender: env.contract.address.to_string(),
        include_expired: None
    };
    let res: Result<cw721_updatable::ApprovalResponse, cosmwasm_std::StdError> = deps.querier.query_wasm_smart(
        cw721_contract.to_string(),
        &query_msg
    );
    if res.is_err() {
        return Err(ContractError::Unapproved {});
    }
    
    // Check time
    let cur_block_id = CUR_BLOCK_ID.load(deps.storage)?;
    let res : cw721_updatable::NftInfoResponse<archid_token::Metadata> = 
        deps.querier.query_wasm_smart(
            cw721_contract.to_string(),
            &cw721_archid::msg::QueryMsg::<archid_token::Extension>::NftInfo { token_id: nft_id.clone() 
        }
    )?;
    let expiry_time = res.extension.expiry.unwrap();
    let now = env.block.time;
    
    if expiry_time < now.seconds() {
        return Err(ContractError::ExpiryLong {});
    }

    let diff_second = Timestamp::from_seconds(expiry_time).minus_seconds(now.seconds()).seconds();
    let diff_block = diff_second.checked_div(5).unwrap();
    let block_div = diff_block.checked_div(u64::from(config.cron_period)).unwrap();
    // let block_div = diff_block % 15;
    let callback_height = block_div * u64::from(config.cron_period);

    if block_div == 0 {
        return Err(ContractError::ExpiryLong {});
    }

    let block_idx = block_div + cur_block_id;
    
    let data = RENEW_JOBS_MAP.may_load(deps.storage, block_idx)?;
    if data.is_none() {
        let _ = RENEW_JOBS_MAP.save(deps.storage, block_idx, &vec![domain_name.clone()]);
    } else {
        let mut new_data = data.unwrap();
        new_data.append(&mut vec![domain_name.clone()]);

        let _ = RENEW_JOBS_MAP.save(deps.storage, block_idx, &new_data);
    }

    // Transfer renew fee to the contract
    let transfer_fee_msg: CosmosMsg = BankMsg::Send { 
        to_address: env.contract.address.to_string(), 
        amount: coins(funds.amount.into(), denom) 
    }.into();

    let msg_to_execute = ExecuteMsg::RenewDomain {
        domain_name: domain_name.to_string()
    };
    // let msg_to_execute = ExecuteMsg::Increment {  };
    let renew_info: RenewInfo = RenewInfo {
        owner: info.sender.to_owned(),
        domain_id: domain_name.to_string(),
        callback_height: state.callback_height + callback_height,
        execute_msg: to_json_binary(&msg_to_execute)?,
        status: 0,
        block_idx: block_idx,
        err_message: None
    };

    let job_id = JOBS.load(deps.storage)?;
    let next_job_id = job_id + 1;
    let _ = RENEW_MAP.save(deps.storage, next_job_id, &renew_info);
    let _ = ACC_JOB_MAP.save(deps.storage, domain_name.to_string(), &next_job_id);
    let _ = JOBS.save(deps.storage, &next_job_id);

    let messages = vec![transfer_fee_msg];

    Ok(Response::new()
        .add_attribute("action", "schedule_auto_renew")
        .add_attribute("domain", domain_name)
        .add_attribute("job_id", job_id.to_string())
        .add_messages(messages)
    )

}

pub fn start_cron_job_callback(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError>
{
    let config = CONFIG.load(deps.storage)?;
    let funds = &info.funds[0];
    let contract_address = env.contract.address.to_string();
    let state = STATE.load(deps.storage)?;
    
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Cancel if it is running
    if state.status == 1 {
        return Err(ContractError::CallbackAlreadyRunning {});
    }

    let fee: cosmos_sdk_proto::cosmos::base::v1beta1::Coin = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: funds.denom.to_string(),
        amount: Uint128::new(config.cron_fee_amount).to_string()
        // amount: Uint128::new(150_000_000_000_000_000).to_string()
    };

    let mut callback_height = env.block.height + u64::from(config.cron_period);
    // let mut callback_height = env.block.height + 12; // Every 1 minute

    let cur_block_id = CUR_BLOCK_ID.load(deps.storage)?;
    let renew_jobs_at_idx: Option<Vec<String>> = RENEW_JOBS_MAP.may_load(deps.storage, cur_block_id)?;
    if renew_jobs_at_idx.is_some() && renew_jobs_at_idx.clone().unwrap().len() > 0 {
        callback_height = env.block.height + 1;
    }

    let regsiter_msg = MsgRequestCallback {
        sender: contract_address.to_string(),
        job_id: 0,
        callback_height: callback_height.clone(),
        contract_address: contract_address.clone(),
        fees: Some(fee)
    };
    let register_stargate_msg = CosmosMsg::Stargate {
        type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
        value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&regsiter_msg)),
    };

    STATE.update(deps.storage, |mut state: crate::state::State| -> Result<_, ContractError> {
        state.status = 1;
        state.callback_height = callback_height;
        Ok(state)
    })?;

    let messages = vec![register_stargate_msg];

    Ok(Response::new()
        .add_attribute("action", "start_cron_callback")
        .add_messages(messages)
    )
}

pub fn stop_cron_job_callback(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError>
{
    let job_id = 0;
    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let contract_address = env.contract.address.to_string();
    let cancel_msg = MsgCancelCallback {
        sender: contract_address.to_string(),
        job_id: job_id.clone(),
        callback_height: state.callback_height.clone(),
        contract_address: contract_address.clone()
    };
    let cancel_stargate_msg = CosmosMsg::Stargate {
        type_url: "/archway.callback.v1.MsgCancelCallback".to_string(),
        value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&cancel_msg)),
    };

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.status = 0;
        state.callback_height = 0;
        Ok(state)
      })?;

    let messages = vec![cancel_stargate_msg];

    Ok(Response::new()
        .add_attribute("action", "stop_cron_callback")
        .add_messages(messages)
    )
}

pub fn cancel_auto_renew (deps: DepsMut, _info: MessageInfo, env: Env, domain_name: String) -> Result<Response, ContractError> {

    let job_id = ACC_JOB_MAP.may_load(deps.storage, domain_name.clone())?;
    if job_id.is_none() {
        return Err(ContractError::NotFoundJobId {});
    }

    let mut renew_info = RENEW_MAP.load(deps.storage, job_id.unwrap())?;
    renew_info.status = 2; // Cancelled
    let _ = RENEW_MAP.save(deps.storage, job_id.unwrap(), &renew_info);

    let data = RENEW_JOBS_MAP.may_load(deps.storage, renew_info.block_idx)?;
    if data.is_none() {
        return Err(ContractError::NotFoundJobId {});
    } else {
        let mut new_data = data.unwrap();
        let index = new_data.iter().position(|x| *x == domain_name).unwrap();
        new_data.remove(index);
        new_data.append(&mut vec![domain_name.clone()]);

        let _ = RENEW_JOBS_MAP.save(deps.storage, renew_info.block_idx, &new_data);
    }

    let _ = RENEW_MAP.save(deps.storage, job_id.unwrap(), &renew_info);

    Ok(Response::new()
        .add_attribute("action", "cancel_auto_renew")
        .add_attribute("domain", domain_name)
    )
}

pub fn deposit_funds(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError>
{
    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let funds = &info.funds[0];
    let deposit_msg : CosmosMsg = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: vec![funds.clone()],
    }.into();

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_message(deposit_msg)
    )
}

pub fn withdraw(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError>
{
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let balance = deps.querier.query_balance(env.contract.address, config.denom)?;
    let withdraw_msg : CosmosMsg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![balance]
    }.into();

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_message(withdraw_msg)
    )
}

pub fn register_ica(deps: DepsMut, info: MessageInfo, env: Env, connection_id: String) -> Result<Response, ContractError>
{
    let from_address = env.contract.address.to_string();
    let state: crate::state::IcaState = ICA_STATE.load(deps.storage)?;
    // let connection_id = state.connection_id;
    let regsiter_msg = MsgRegisterInterchainAccount {
        contract_address: from_address.clone(),
        connection_id: connection_id.clone(),
    };
    let register_stargate_msg = CosmosMsg::Stargate {
        type_url: "/archway.cwica.v1.MsgRegisterInterchainAccount".to_string(),
        value: Binary::from(prost::Message::encode_to_vec(&regsiter_msg)),
    };
    Ok(Response::new()
        .add_attribute("action", "register")
        .add_attribute("account_owner", from_address)
        .add_attribute("connection_id", connection_id)
        .add_message(register_stargate_msg))
}

const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

pub fn vote(deps: Deps, env: Env, proposal_id: u64, option: i32, connection_id: String, tiny_timeout: bool) -> Result<Response, ContractError> 
{
    let state = ICA_STATE.load(deps.storage)?;
    // let connection_id = state.connection_id;
    let from_address = env.contract.address.to_string();
    let ica_address = state.ica_address;
    let vote_msg = MsgVote {
      proposal_id: proposal_id,
      voter: ica_address.clone(),
      option: option,
    };
    let vote_msg_stargate_msg = prost_types::Any {
      type_url: "/cosmos.gov.v1.MsgVote".to_string(),
      value: vote_msg.encode_to_vec(),
    };
    let timeout = if tiny_timeout {
      1
    } else {
      DEFAULT_TIMEOUT_SECONDS
    };
    let sendtx_msg = MsgSendTx {
      contract_address: from_address.clone(),
      connection_id: connection_id.clone(),
      msgs: vec![vote_msg_stargate_msg],
      memo: "sent from contract".to_string(),
      timeout: timeout,
    };
    let sendtx_stargate_msg = CosmosMsg::Stargate {
      type_url: "/archway.cwica.v1.MsgSendTx".to_string(),
      value: Binary::from(prost::Message::encode_to_vec(&sendtx_msg)),
    };
    Ok(Response::new()
      .add_attribute("action", "send")
      .add_message(sendtx_stargate_msg))
}