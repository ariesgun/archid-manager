use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use crate::{
    msg::{DomainDefaultResponse, GetCountResponse, QueryErrorsRequest, QueryErrorsResponse, QueryMsg, RenewJobsMapResponse, RenewMapResponse},
    state::{ACC_JOB_MAP, DEFAULT_ID, RENEW_JOBS_MAP, RENEW_MAP, STATE}
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
        QueryMsg::QueryRenewMap { domain_name} => to_json_binary(&query_renew_map(deps, env, domain_name)?),
        QueryMsg::QueryRenewJobsMap { block_id} => to_json_binary(&query_renew_job_map(deps, env, block_id)?),
    }
}

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { state })
}

pub fn query_domain_default(deps: Deps, address: Addr) -> StdResult<DomainDefaultResponse> {
    let default_id = DEFAULT_ID.load(deps.storage, address);
    if default_id.is_err() {
        Ok(DomainDefaultResponse { domain_id: "".to_string()})
    } else {
        Ok(DomainDefaultResponse { domain_id: default_id.unwrap() })
    }
}

pub fn query_renew_map(deps: Deps, env: Env, domain_name: String) -> StdResult<RenewMapResponse> {
    
    let job_id = ACC_JOB_MAP.may_load(deps.storage, domain_name)?;
    if job_id.is_none() {
        Ok(RenewMapResponse { renew_info: None })
    } else {
        let renew_info = RENEW_MAP.load(deps.storage, job_id.unwrap())?;
        Ok(RenewMapResponse { renew_info: Some(renew_info) })
    }
}

pub fn query_renew_job_map(deps: Deps, env: Env, block_id: u64) -> StdResult<RenewJobsMapResponse> {
    let renew_jobs = RENEW_JOBS_MAP.may_load(deps.storage, block_id)?;
    if renew_jobs.is_none() {
        Ok(RenewJobsMapResponse { renew_jobs: vec![]})
    } else {
        Ok(RenewJobsMapResponse { renew_jobs: renew_jobs.unwrap() })
    }
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

#[cfg(test)]
mod tests {
    use crate::msg::GetCountResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr};

    #[test]
    fn query_nonexisting_default_domain() {
        let mut deps = mock_dependencies();
        let user = Addr::unchecked("user");

        let res = query_domain_default(deps.as_ref(), user);
        println!("{:?}", res);

    }
}