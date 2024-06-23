use std::str::FromStr;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::{msg::InstantiateMsg, state::{Config, State, CONFIG, CUR_BLOCK_ID, JOBS, STATE}, ContractError};


pub fn instantiate_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        status: 0,
        renew_status: 0,
        owner: info.sender.clone(),
        callback_height: 0,
    };
    STATE.save(deps.storage, &state)?;
    let cur_job_id = 2;
    CUR_BLOCK_ID.save(deps.storage, &cur_job_id)?;
    CONFIG.save(
        deps.storage,
        &&Config {
            owner: info.sender.clone(),
            archid_registry_addr: msg.archid_registry_addr,
            cw721_archid_addr: msg.cw721_archid_addr,
            denom: msg.denom,
            start_block: env.block.height,
            cron_period: msg.cron_period, // 7 days = 120_000
            cron_fee_amount: 270_000_000_000_000_000, // reservation fee
            cost_per_year: Uint128::from_str(msg.cost_per_year.as_str())?.u128(),
            gas_fee: 150_000_000_000_000_000,
        }
    )?;
    
    let job_init = 0;
    JOBS.save(deps.storage, &job_init)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}