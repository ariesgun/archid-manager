use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub archid_registry_addr: Addr,
    pub cw721_archid_addr: Addr,
    pub denom: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RenewInfo {
    pub owner: Addr,
    pub domain_id: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONFIG: Item<Config> = Item::new("config");
pub const JOBS: Item<u64> = Item::new("jobs");
pub const RENEW_MAP: Map<u64, RenewInfo> = Map::new("renew_info");
pub const DEFAULT_ID: Map<Addr, String> = Map::new("default_id");