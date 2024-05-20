use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    #[payable]
    Register {
        job_id: u64
    },
    #[payable]
    MintDomain {
        domain_name: String
    },
    #[payable]
    RenewDomain {
        domain_name: String
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[derive(cw_orch::QueryFns)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub enum SudoMsg {
  Callback { job_id: u64 },
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