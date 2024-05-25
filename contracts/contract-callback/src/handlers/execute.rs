use cosmwasm_std::{to_json_binary, Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128, WasmMsg};
use crate::{msg::{ExecuteMsg, MsgRequestCallback}, state::{RenewInfo, CONFIG, DEFAULT_ID, JOBS, RENEW_MAP, STATE}, ContractError};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps),
        ExecuteMsg::Reset { count } => reset(deps, info, count),
        ExecuteMsg::MintDomain { domain_name } => mint_domain(deps, info, env, domain_name),
        ExecuteMsg::RenewDomain { domain_name } => renew_domain(deps, info, env, domain_name),
        ExecuteMsg::ScheduleAutoRenew { domain_name } => schedule_auto_renew(deps, info, env, domain_name),
        ExecuteMsg::SetDefault { domain_name } => set_default(deps, info, env, domain_name)
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

pub fn mint_domain(deps: DepsMut, info: MessageInfo, _env: Env, domain_name: String) -> Result<Response, ContractError> {
    // let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    let cost_per_year: u128 = 1_000_000_000_000_000_000;
    // let denom = "aconst";

    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;

    let res = cw_utils::must_pay(&info, &denom)?;
    let registration: u64 =
        u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    if registration < 1 {
        return Err(ContractError::InvalidPayment { amount: res });
    }

    // Create registration msg
    let register_msg = archid_registry::msg::ExecuteMsg::Register {
        name: domain_name.clone().into(),
    };
    // Do registration
    let register_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.clone().into(),
        msg: to_json_binary(&register_msg)?,
        funds: vec![Coin {
            denom: denom.into(),
            amount: Uint128::from(cost_per_year), // E.g. register for 1 year
        }],
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
    // let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    // let denom = "aconst";

    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;

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
    // let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    // let cw721_contract = "archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008";

    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;

    let cost_per_year: u128 = 1_000_000_000_000_000_000;
    // let denom = "aconst";

    let nft_id = domain_name.clone() + ".arch";

    let res = cw_utils::must_pay(&info, &String::from(denom.clone()))?;
    let registration: u64 =
        u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    if registration < 1 {
        return Err(ContractError::InvalidPayment { amount: res });
    }

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

    // Update resolver
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

    let transfer_nft_msg_2 = archid_token::ExecuteMsg::TransferNft { 
        recipient: info.sender.to_string(),
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
        .add_messages(messages)
    )
}

pub fn schedule_auto_renew(deps: DepsMut, info: MessageInfo, env: Env, domain_name: String) -> Result<Response, ContractError> {

    // Check if approve has been done
    // let cw721_contract = "archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008";

    let config = CONFIG.load(deps.storage)?;
    let registry_contract = config.archid_registry_addr;
    let cw721_contract = config.cw721_archid_addr;
    let denom = config.denom;
    
    let nft_id = domain_name.to_string() + ".arch";

    let query_msg:  archid_token::QueryMsg<archid_token::Extension>  = archid_token::QueryMsg::Approval { 
        token_id: nft_id.to_string(),
        spender: env.contract.address.to_string(),
        include_expired: None
    };

    let _: cw721_updatable::ApprovalResponse = deps.querier.query_wasm_smart(
        cw721_contract.to_string(),
        &query_msg
    )?;

    let contract_address = env.contract.address.to_string();
    let funds = &info.funds[0];
    let job_id = JOBS.load(deps.storage)? + 1;
    let callback_height = env.block.height + 10;

    let fee = cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: funds.denom.to_string(),
        amount: funds.amount.to_string()
    };

    let regsiter_msg = MsgRequestCallback {
        sender: contract_address.to_string(),
        job_id: job_id.clone(),
        callback_height: callback_height.clone(),
        contract_address: contract_address.clone(),
        fees: Some(fee)
    };
    let register_stargate_msg = CosmosMsg::Stargate {
        type_url: "/archway.callback.v1.MsgRequestCallback".to_string(),
        value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&regsiter_msg)),
    };

    let renew_info: RenewInfo = RenewInfo {
        owner: info.sender.to_owned(),
        domain_id: domain_name.to_string(),
    };

    let _ = RENEW_MAP.save(deps.storage, job_id, &renew_info);

    // let callback_msg = MsgSubscribeToError  {
    //     sender: contract_address.to_string(),
    //     contract_address: contract_address.clone(),
    //     fees: None
    // };
    // let subscribe_error_msg = CosmosMsg::Stargate {
    //     type_url: "/archway.cwerrors.v1.MsgSubscribeToError ".to_string(),
    //     value: Binary::from(::cosmos_sdk_proto::traits::Message::encode_to_vec(&callback_msg)),
    // };

    let messages = vec![register_stargate_msg];
    
    let _ = JOBS.save(deps.storage, &job_id);

    Ok(Response::new()
        .add_attribute("action", "schedule_auto_renew")
        .add_messages(messages)
    )

}