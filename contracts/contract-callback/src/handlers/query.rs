use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

use crate::{msg::{GetCountResponse, QueryMsg}, state::STATE};


pub fn query_handler(
    deps: Deps, 
    env: Env,
    msg: QueryMsg) -> StdResult<Binary> 
{
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&count(deps)?),
    }
}

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}