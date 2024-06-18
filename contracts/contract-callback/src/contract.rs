use cosmwasm_std::{entry_point, Reply};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:contract-callback";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::handlers::{execute_handler, instantiate_handler, query_handler, sudo_handler};
use crate::state::{RENEW_MAP, STATE};


#[cfg_attr(feature = "export", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    instantiate_handler(deps, env, info, msg)
}

#[cfg_attr(feature = "export", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    execute_handler(deps, env, info, msg)
}

#[cfg_attr(feature = "export", entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query_handler(deps, env, msg)
}

#[cfg_attr(feature = "export", entry_point)]
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response, ContractError> {
  sudo_handler(deps, env, msg)
}

#[cfg_attr(feature = "export", entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    handle_reply(deps, env, msg)
}
fn handle_reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    let job_id = msg.id;

    if job_id == 0 {
        let _ = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.status = 2;
            Ok(state)
        });
        Ok(Response::new())
    } else if job_id == 1 {

        if msg.result.is_err() {
            let _ = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state.renew_status = 2;
                Ok(state)
            });
        } else {
            let _ = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state.renew_status = 1;
                Ok(state)
            });
        }

        Ok(Response::new())
    } else {
        let mut renew_info = RENEW_MAP.load(deps.storage, job_id)?;
        renew_info.callback_height = env.block.height;
        
        if msg.result.is_err() {
            renew_info.err_message = Some(msg.result.unwrap_err());
            renew_info.status = 999;
        } else {
            renew_info.status = 1;
        }
        let _ = RENEW_MAP.save(deps.storage, job_id, &renew_info);

        Ok(Response::new())
    }
}

#[cfg_attr(feature = "export", entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from a compatible contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same contract type").into());
    }
    // note: it's better to do a proper semver comparison, but a string comparison *usually* works
    #[allow(clippy::cmp_owned)]
    if *ver.version >= *CONTRACT_VERSION {
        return Err(StdError::generic_err("Cannot upgrade from a newer contract version").into());
    }
    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // do any required state migrations...
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use crate::msg::GetCountResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr, SubMsgResult};
    use serde_json::to_vec;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let cw721_addr = Addr::unchecked("cw721_addr");
        let archid_registry_addr = Addr::unchecked("archid_registry");

        let msg = InstantiateMsg { 
            count: 17, 
            cw721_archid_addr: cw721_addr,
            archid_registry_addr: archid_registry_addr,
            denom: "aarch".to_string(),
            cost_per_year: "250000000000000000".to_string(),
         };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_json(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn handle_reply_error() {
        let mut deps = mock_dependencies();

        let error_msg = String::from("Something went wrong");
        let result = SubMsgResult::Err(error_msg);

        let msg = Reply { id: 1, result: result.clone() };

        println!("Error {}", msg.clone().result.clone().unwrap_err());
        println!("Error {:?}", result.unwrap_err());

        let _ = reply(deps.as_mut(), mock_env(), msg);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let cw721_addr = Addr::unchecked("cw721_addr");
        let archid_registry_addr = Addr::unchecked("archid_registry");

        let msg = InstantiateMsg { 
            count: 17, 
            cw721_archid_addr: cw721_addr,
            archid_registry_addr: archid_registry_addr,
            denom: "aarch".to_string(),
            cost_per_year: "250000000000000000".to_string(),
         };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_json(&res).unwrap();
        // assert_eq!(18, value.count);
    }
}
