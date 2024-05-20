use contract_callback::{msg::InstantiateMsg, AppContract, AppExecuteMsgFns, AppQueryMsgFns};
use cw_orch::{anyhow, daemon::networks::CONSTANTINE_3, prelude::*};

pub fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    env_logger::init();

    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let chain = Daemon::builder()
        // set the network to use
        .chain(cw_orch::daemon::networks::CONSTANTINE_3) // chain parameter
        .build()
        .unwrap();

    let counter = AppContract::new(chain);
    let res = counter.upload();
    assert!(res.is_ok());

    let res = counter.instantiate(
        &InstantiateMsg { count: 12 }, 
        Some(&counter.get_chain().sender()), 
        None
    );
    assert!(res.is_ok());

    println!("Count {:?}", counter.get_count()?);

    Ok(())
}
