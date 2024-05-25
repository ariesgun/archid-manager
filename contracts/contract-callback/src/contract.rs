use cosmwasm_std::{entry_point, to_json_binary, wasm_execute, CosmosMsg, Reply, WasmMsg};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:contract-callback";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::handlers::{execute_handler, instantiate_handler, query_handler, sudo_handler};
use crate::state::CONFIG;


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
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        1u64 => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}
fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Handle the msg data and save the contract address
    // See: https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs
    let data = msg.result.into_result().map_err(StdError::generic_err)?;
    // Search for the transfer event
    // If there are multiple transfers, you will need to find the right event to handle
    let mint_event = data
        .events
        .iter()
        .find(|e| {
            e.attributes
                .iter()
                .any(|attr| attr.key == "action" && attr.value == "mint")
        })
        .ok_or_else(|| StdError::generic_err(format!("unable to find transfer action")))?;
    // // Do whatever you want with the attributes in the transfer event
    // // Reference to the full event: https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L239-L244

    // let cw721_contract = "archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008";
    let config = CONFIG.load(deps.storage)?;
    let cw721_contract = config.cw721_archid_addr;

    let domain_name = mint_event.attributes.iter().find(|attr| attr.key == "token_id").unwrap().value.clone();
    let receipent = mint_event.attributes.iter().find(|attr| attr.key == "domain_minter").unwrap().value.clone();

    println!("hello {}", receipent);
    let transfer_msg: archid_token::ExecuteMsg = archid_token::ExecuteMsg::TransferNft { 
        recipient: receipent.to_string(),
        token_id: domain_name.into(),
    };

    let transfer_nft: CosmosMsg = WasmMsg::Execute { 
        contract_addr: cw721_contract.to_string(),
        msg: to_json_binary(&transfer_msg)?,
        funds: vec![] 
    }.into();

    Ok(Response::new()
        .add_message(transfer_nft)
    )
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
    use cosmwasm_std::{coins, from_json, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let cw721_addr = Addr::unchecked("cw721_addr");
        let archid_registry_addr = Addr::unchecked("archid_registry");

        let msg = InstantiateMsg { 
            count: 17, 
            cw721_archid_addr: cw721_addr,
            archid_registry_addr: archid_registry_addr,
            denom: "aarch".to_string()
         };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
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
            denom: "aarch".to_string()
         };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let cw721_addr = Addr::unchecked("cw721_addr");
        let archid_registry_addr = Addr::unchecked("archid_registry");

        let msg = InstantiateMsg { 
            count: 17, 
            cw721_archid_addr: cw721_addr,
            archid_registry_addr,
            denom: "aarch".to_string()
         };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
