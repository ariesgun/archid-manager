use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{RenewInfo, State};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: u64,
    pub cw721_archid_addr: Addr,
    pub archid_registry_addr: Addr,
    pub denom: String
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Increment {},
    #[payable]
    MintDomain {
        domain_name: String
    },
    #[payable]
    RenewDomain {
        domain_name: String
    },
    #[payable]
    ScheduleAutoRenew {
        domain_name: String
    },
    CancelAutoRenew {
        domain_name: String
    },
    SetDefault {
        domain_name: String
    },
    #[payable]
    StartCronJob {
    },
    StopCronJob {
    },
    #[payable]
    Deposit {

    },
    Withdraw {
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[derive(cw_orch::QueryFns)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(QueryErrorsResponse)]
    QueryErrors {},
    #[returns(DomainDefaultResponse)]
    QueryDomainDefault {
        address: Addr
    },
    #[returns(RenewMapResponse)]
    QueryRenewMap {
        domain_name: String
    },
    #[returns(RenewJobsMapResponse)]
    QueryRenewJobsMap {
        block_id: u64
    },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub state: State,
}

#[cw_serde]
pub struct DomainDefaultResponse {
    pub domain_id: String,
}

#[cw_serde]
pub struct RenewMapResponse {
    pub renew_info: Option<RenewInfo>,
}

#[cw_serde]
pub struct RenewJobsMapResponse {
    pub renew_jobs: Vec<String>,
}

#[cw_serde]
pub struct SudoError {
    module_name: String,
    error_code: u32,
    contract_address: String,
    input_payload: String,
    error_message: String,
}

#[cw_serde]
pub struct QueryErrorsResponse {
    errors: Vec<SudoError>
}

#[cw_serde]
pub enum SudoMsg {
  Callback { job_id: u64 },
  Error {
    module_name: String,
    error_code: u32,
    contract_address: String,
    input_payload: String,
    error_message: String,
  }
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestCallback {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub job_id: u64,
    #[prost(uint64, tag = "4")]
    pub callback_height: u64,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub fees: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelCallback {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub job_id: u64,
    #[prost(uint64, tag = "4")]
    pub callback_height: u64,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubscribeToError {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub fees: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorsRequest {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}

