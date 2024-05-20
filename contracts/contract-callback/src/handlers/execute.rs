use cosmwasm_std::{to_json_binary, wasm_execute, Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128, WasmMsg};
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
        ExecuteMsg::MintDomain { domain_name } => mint_domain(deps, info, env, domain_name),
        ExecuteMsg::RenewDomain { domain_name } => renew_domain(deps, info, env, domain_name),
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

pub fn mint_domain(_deps: DepsMut, info: MessageInfo, _env: Env, domain_name: String) -> Result<Response, ContractError> {
    let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    let cost_per_year: u128 = 1_000_000_000_000_000_000;
    let denom = "aconst";

    // Create registration msg
    let register_msg = archid_registry::msg::ExecuteMsg::Register {
        name: domain_name.clone().into(),
    };

    let res = cw_utils::must_pay(&info, &String::from(denom))?;
    let registration: u64 =
        u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    if registration < 1 {
        return Err(ContractError::InvalidPayment { amount: res });
    }

    // Do registration
    let register_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.into(),
        msg: to_json_binary(&register_msg)?,
        funds: vec![Coin {
            denom: denom.into(),
            amount: Uint128::from(cost_per_year), // E.g. register for 1 year
        }],
        }
    .into();

    let submessage = SubMsg::reply_on_success(register_resp, 1u64);

    Ok(Response::new()
        .add_attribute("action", "mint_domain")
        .add_submessage(submessage)
    )
}

pub fn renew_domain(_deps: DepsMut, info: MessageInfo, env: Env, domain_name: String) -> Result<Response, ContractError> {
    let registry_contract = "archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r";
    let cw721_contract = "archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008";

    let cost_per_year: u128 = 250_000_000_000_000_000;
    let denom = "aconst";

    let nft_id = domain_name.clone() + ".arch";

    let res = cw_utils::must_pay(&info, &String::from(denom))?;
    let registration: u64 =
        u64::try_from(((res.checked_div(cost_per_year.into())).unwrap()).u128()).unwrap();
    if registration < 1 {
        return Err(ContractError::InvalidPayment { amount: res });
    }

    let transfer_nft_msg = archid_token::ExecuteMsg::TransferNft { 
        recipient: env.contract.address.to_string(),
        token_id: nft_id.to_string() 
    };

    // Do registration
    let transfer_execute: CosmosMsg = WasmMsg::Execute {
        contract_addr: cw721_contract.into(),
        msg: to_json_binary(&transfer_nft_msg)?,
        funds: vec![]
    }.into();

    // // Need to approve nft first
    // let approve_nft_msg = archid_token::ExecuteMsg::Approve { 
    //     spender: env.contract.address.to_string(), 
    //     token_id: domain_name.to_string(), 
    //     expires: None
    // };

    // // Do registration
    // let approve_execute: CosmosMsg = WasmMsg::Execute {
    //     contract_addr: cw721_contract.into(),
    //     msg: to_json_binary(&approve_nft_msg)?,
    //     funds: vec![]
    // }.into();

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

    let transfer_nft_msg_2 = archid_token::ExecuteMsg::TransferNft { 
        recipient: info.sender.to_string(),
        token_id: nft_id.to_string() 
    };

    // Do registration
    let transfer_execute_2: CosmosMsg = WasmMsg::Execute {
        contract_addr: cw721_contract.into(),
        msg: to_json_binary(&transfer_nft_msg_2)?,
        funds: vec![]
    }.into();

    let messages = vec![transfer_execute, renew_execute, transfer_execute_2];

    Ok(Response::new()
        .add_attribute("action", "renew_domain")
        .add_messages(messages)
    )
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