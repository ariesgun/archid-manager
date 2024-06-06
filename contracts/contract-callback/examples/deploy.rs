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
        &InstantiateMsg { 
            count: 5,
            cw721_archid_addr: Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
            archid_registry_addr: Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
            denom: "aconst".to_string()
        }, 
        Some(&counter.get_chain().sender()), 
        None
    );
    assert!(res.is_ok());

    println!("Count {:?}", counter.get_count()?);

    Ok(())
}
