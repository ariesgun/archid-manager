pub mod contract;
mod error;
mod handlers;
pub mod helpers;
// pub mod integration_tests;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

// Only used for easier importing
pub use crate::msg::{ExecuteMsgFns as AppExecuteMsgFns, QueryMsgFns as AppQueryMsgFns};

// This interface file should not land inside the wasm
#[cfg(not(target_arch = "wasm32"))]
pub mod interface;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::AppContract;