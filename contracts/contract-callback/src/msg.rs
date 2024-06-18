use std::any;

use cosmos_sdk_proto::ibc;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, IbcPacket};
use prost_types::Any;

use crate::state::{IcaState, RenewInfo, State};

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
    RegisterIca {
        connection_id: String
    },
    Vote {
        proposal_id: u64,
        option: i32,
        connection_id: String,
        tiny_timeout: bool
    },
}

#[cw_serde]
#[derive(QueryResponses)]
#[derive(cw_orch::QueryFns)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    #[returns(GetIcaStateResponse)]
    GetIcaState {},
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
pub struct GetIcaStateResponse {
    pub state: IcaState,
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
  Ica {
    account_registered: Option<AccountRegistered>,
    tx_executed: Option<ICAResponse>,
},
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


#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendTx  {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(string, tag = "4")]
    pub memo: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub timeout: u64,
}

#[cw_serde]
pub struct ICAResponse {
    pub packet: RequestPacket,
    pub data: Binary, 
}

#[cw_serde]
pub struct RequestPacket {
    pub sequence: Option<u64>,
    pub source_port: Option<String>,
    pub source_channel: Option<String>,
    pub destination_port: Option<String>,
    pub destination_channel: Option<String>,
    pub data: Option<Binary>,
    pub timeout_height: Option<RequestPacketTimeoutHeight>,
    pub timeout_timestamp: Option<u64>,
}

#[cw_serde]
pub struct RequestPacketTimeoutHeight {
    pub revision_number: Option<u64>,
    pub revision_height: Option<u64>,
}

#[cw_serde]
pub struct AccountRegistered {
    pub counterparty_address: String,
}

/// MsgVote defines a message to cast a vote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgVote {
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(enumeration = "VoteOption", tag = "3")]
    pub option: i32,
}

/// VoteOption enumerates the valid vote options for a given governance proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteOption {
    /// VOTE_OPTION_UNSPECIFIED defines a no-op vote option.
    Unspecified = 0,
    /// VOTE_OPTION_YES defines a yes vote option.
    Yes = 1,
    /// VOTE_OPTION_ABSTAIN defines an abstain vote option.
    Abstain = 2,
    /// VOTE_OPTION_NO defines a no vote option.
    No = 3,
    /// VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
    NoWithVeto = 4,
}
impl VoteOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteOption::Unspecified => "VOTE_OPTION_UNSPECIFIED",
            VoteOption::Yes => "VOTE_OPTION_YES",
            VoteOption::Abstain => "VOTE_OPTION_ABSTAIN",
            VoteOption::No => "VOTE_OPTION_NO",
            VoteOption::NoWithVeto => "VOTE_OPTION_NO_WITH_VETO",
        }
    }
}