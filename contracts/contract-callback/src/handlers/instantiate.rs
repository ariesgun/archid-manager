use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::{msg::InstantiateMsg, state::{Config, IcaState, State, CONFIG, CUR_BLOCK_ID, ICA_STATE, JOBS, STATE}, ContractError};


pub fn instantiate_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        status: 0,
        renew_status: 1,
        owner: info.sender.clone(),
        callback_height: 0,
    };
    STATE.save(deps.storage, &state)?;
    let ica_state = IcaState {
        owner: info.sender.clone(),
        connection_id: "channel-99".to_string(),
        ica_address: "".to_string(),
        voted: false,
        errors: "".to_string(),
        timeout: false,
    };
    ICA_STATE.save(deps.storage, &ica_state)?;
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
            cron_period: 120_000, // 7 days
            cron_fee_amount: 270_000_000_000_000_000 // reservation fee
        }
    )?;
    
    let job_init = 0;
    JOBS.save(deps.storage, &job_init)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}