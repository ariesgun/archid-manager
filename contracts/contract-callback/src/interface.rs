use cw_orch::environment::ChainInfoOwned;
use cw_orch::{interface, prelude::*};


pub const CONTRACT_ID: &str = "contract_callback";

#[interface(
    crate::msg::InstantiateMsg,
    crate::msg::ExecuteMsg,
    crate::msg::QueryMsg,
    crate::msg::MigrateMsg,
    id = CONTRACT_ID
)]
pub struct AppContract;


impl<Chain> Uploadable for AppContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path(CONTRACT_ID)
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
            .with_migrate(crate::contract::migrate),
        )
    }
}