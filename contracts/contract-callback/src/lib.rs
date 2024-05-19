pub mod contract;
mod error;
mod handlers;
pub mod helpers;
pub mod integration_tests;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

// Only used for easier importing
pub use crate::msg::{ExecuteMsgFns as AppExecuteMsgFns, QueryMsgFns as AppQueryMsgFns};

#[cw_orch::interface(
    crate::msg::InstantiateMsg,
    crate::msg::ExecuteMsg,
    crate::msg::QueryMsg,
    crate::msg::MigrateMsg
)]
pub struct AppContract;

// This interface file should not land inside the wasm
// #[cfg(not(target_arch = "wasm32"))]
// mod interface;