use contract_callback::{msg::InstantiateMsg, AppContract, AppExecuteMsgFns, AppQueryMsgFns};
use cosmwasm_std::{coin, to_json_binary, BankMsg, BlockInfo, Timestamp, Uint128};
use cosmwasm_storage::ReadonlySingleton;
use cw_orch::{anyhow, daemon::{networks::CONSTANTINE_3, DaemonError}, prelude::*};
use cw_utils::Scheduled;

pub fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    env_logger::init();

    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let chain = Daemon::builder()
        // set the network to use
        .chain(cw_orch::daemon::networks::CONSTANTINE_3) // chain parameter
        .build()
        .unwrap();

    let counter = AppContract::new(chain.clone());

    // Incrementing count
    println!("ID {:?}", counter.code_id());
    
    counter.deposit(&[coin(5_000_000_000_000_000_000_u128, "aconst")])?;
    let res = counter.start_cron_job(
        &[Coin {
                    amount: Uint128::new(0_500_000_000_000_000_000),
                    denom: "aconst".to_string()
                }]
    )?;

    println!("Status {:?}", counter.get_count()?);

    // let res = counter.stop_cron_job()?;
    // let res = counter.withdraw()?;
    // println!("Res {:?}", res);

    Ok(())
}
