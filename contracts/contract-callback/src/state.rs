use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, SubMsgResult};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: u64,
    pub status: u8,
    pub renew_status: u8,
    pub owner: Addr,
    pub callback_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub archid_registry_addr: Addr,
    pub cw721_archid_addr: Addr,
    pub denom: String,
    pub start_block: u64,
    pub cron_period: u32,
    pub cron_fee_amount: u128
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RenewInfo {
    pub owner: Addr,
    pub domain_id: String,
    pub callback_height: u64,
    pub execute_msg: Binary,
    pub status: u64,
    pub block_idx: u64,
    pub err_message: Option<String>
}

pub const STATE: Item<State> = Item::new("state");
pub const CONFIG: Item<Config> = Item::new("config");

pub const JOBS: Item<u64> = Item::new("jobs");
pub const ACC_JOB_MAP: Map<String, u64> = Map::new("acc_job_map");
pub const RENEW_MAP: Map<u64, RenewInfo> = Map::new("renew_info");
// Block Height -> list of domains to renew
pub const RENEW_JOBS_MAP: Map<u64, Vec<String>> = Map::new("renew_jobs_map");
pub const CUR_BLOCK_ID: Item<u64> = Item::new("cur_block_id");

pub const DEFAULT_ID: Map<Addr, String> = Map::new("default_id");