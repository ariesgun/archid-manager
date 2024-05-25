use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::{msg::InstantiateMsg, state::{Config, State, CONFIG, JOBS, STATE}, ContractError};


pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;
    CONFIG.save(
        deps.storage,
        &&Config {
            archid_registry_addr: msg.archid_registry_addr,
            cw721_archid_addr: msg.cw721_archid_addr,
            denom: msg.denom
        }
    )?;
    
    let job_init = 0;
    JOBS.save(deps.storage, &job_init)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}