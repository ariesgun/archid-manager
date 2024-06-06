use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use crate::{
    msg::{DomainDefaultResponse, GetCountResponse, QueryErrorsRequest, QueryErrorsResponse, QueryMsg, RenewMapResponse},
    state::{DEFAULT_ID, RENEW_MAP, STATE}
};


pub fn query_handler(
    deps: Deps, 
    env: Env,
    msg: QueryMsg) -> StdResult<Binary> 
{
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&count(deps)?),
        QueryMsg::QueryErrors {} => to_json_binary(&query_cw_errors(deps, env)?),
        QueryMsg::QueryDomainDefault {address} => to_json_binary(&query_domain_default(deps, address)?),
        QueryMsg::QueryRenewMap { job_id} => to_json_binary(&query_renew_map(deps, env, job_id)?),
    }
}

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

pub fn query_domain_default(deps: Deps, address: Addr) -> StdResult<DomainDefaultResponse> {
    let default_id = DEFAULT_ID.load(deps.storage, address)?;
    Ok(DomainDefaultResponse { domain_id: default_id })
}

pub fn query_renew_map(deps: Deps, env: Env, job_id: u64) -> StdResult<RenewMapResponse> {
    let renew_info = RENEW_MAP.load(deps.storage, job_id)?;
    Ok(RenewMapResponse { renew_info })
}

pub fn query_cw_errors(
    deps: Deps,
    env: Env,
  ) -> StdResult<Option<QueryErrorsResponse>> 
{
    let contract_address = env.contract.address.to_string();
    let msg = QueryErrorsRequest {
        contract_address: contract_address.clone(),
    };
    let res = deps.querier.query(&cosmwasm_std::QueryRequest::Stargate {
        path: "/archway.cwerrors.v1.QueryErrorsRequest".to_string(),
        data: Binary::from(prost::Message::encode_to_vec(&msg)),
    })?;

    Ok(res)
}