use contract_callback::{msg::InstantiateMsg, AppContract, AppExecuteMsgFns, AppQueryMsgFns};
use cw_orch::{anyhow, prelude::*};
use networks::{archway::ARCHWAY_NETWORK, ChainKind};

pub const CONSTANTINE_3: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "constantine-3",
    gas_denom: "aconst",
    gas_price: 1000000000000.0,
    grpc_urls: &["https://grpc.constantine.archway.io:443"],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.constantine.archway.io"),
    fcd_url: None,
};

pub fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok(); 
    env_logger::init();

    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let chain = Daemon::builder()
        // set the network to use
        .chain(CONSTANTINE_3) // chain parameter
        .build()
        .unwrap();

    let counter = AppContract::new(chain);
    let res = counter.upload();
    assert!(res.is_ok());

    let res = counter.instantiate(
        &InstantiateMsg { 
            count: 10,
            cw721_archid_addr: Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
            archid_registry_addr: Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
            denom: "aconst".to_string(),
            cost_per_year: "250000000000000000".to_string(),
            // cron_period: 120_000, // 7 days
            cron_period: 36, // 3 mins
        }, 
        Some(&counter.get_chain().sender()), 
        None
    );
    assert!(res.is_ok());

    println!("Count {:?}", counter.get_count()?);

    Ok(())
}
