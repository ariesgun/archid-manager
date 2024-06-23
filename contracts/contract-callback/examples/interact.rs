use archid_registry::state::Config;
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
    // counter.increment();
    // counter.increment();
    // counter.increment();
    // println!("Count {:?}", counter.get_count()?);

    let domain_name = "testdomainx16";

    // let res = counter.mint_domain(
    //     domain_name.to_string(),
    //     &[Coin {
    //         amount: Uint128::new(0_250_000_000_000_000_000),
    //         denom: "aconst".to_string()
    //     }]
    // )?;
    // println!("Res {:?}", res);

    let nft_id = domain_name.to_string() + ".arch";

    // let approve_msg: cw721_archid::ExecuteMsg<Option<Empty>, Empty> = cw721_archid::msg::ExecuteMsg::<Option<Empty>, Empty>::Approve {
    //     spender: counter.addr_str()?.to_string(),
    //     token_id: nft_id.clone(),
    //     expires: None
    // };
    // let res = chain.execute(
    //     &approve_msg,
    //     &[], 
    //     &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
    // )?;

    // let res : Result<cw721_updatable::ApprovalResponse, DaemonError> = chain.wasm_querier().smart_query(
    //     &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
    //     &cw721_archid::msg::QueryMsg::<archid_token::Extension>::Approval { spender: "archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq".to_string(), token_id: nft_id.clone(), include_expired: None }
    // );
    // println!("{:?}", res.is_ok());
    // println!("{:?}", res);

    let res : cw721_updatable::NftInfoResponse<archid_token::Metadata> = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::NftInfo { token_id: nft_id.clone() }
    )?;
    println!("{:?}", res);

    let res : cw721_updatable::TokensResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::Tokens { owner: chain.sender().to_string(), start_after: None, limit: None }
    )?;
    println!("{:?}", res);

    let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: nft_id.clone() }
    )?;
    println!("{:?}", res);
    let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "testdomainy1.arch".to_string() }
    )?;
    println!("{:?}", res);
    let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "testdomainx11.arch".to_string() }
    )?;
    println!("{:?}", res);
    let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "testdomainx12.arch".to_string() }
    )?;
    println!("{:?}", res);

    // // Renew domain
    // let res = counter.renew_domain(
    //     domain_name.to_string(),
    //     &[Coin {
    //         amount: Uint128::new(250_000_000_000_000_000),
    //         denom: "aconst".to_string()
    //     }]
    // )?;
    // println!("Res {:?}", res);
    
    // counter.deposit(&[coin(2_000_000_000_000_000_000_u128, "aconst")])?;
    // let res = counter.start_cron_job(
    //     &[Coin {
    //                 amount: Uint128::new(0_500_000_000_000_000_000),
    //                 denom: "aconst".to_string()
    //             }]
    // )?;
    // println!("Start Cron Job: {:?}", res);

    // let res = counter.schedule_auto_renew( 
    //     domain_name.to_string(), 
    //     &[Coin {
    //         amount: Uint128::new(0_400_000_000_000_000_000),
    //         denom: "aconst".to_string()
    //     }]
    // );

    println!("Status {:?}", counter.get_count()?);

    // let res = counter.stop_cron_job()?;
    // let res = counter.withdraw()?;
    // println!("Res {:?}", res);

    let res = counter.query_renew_jobs_map(2);
    println!("Res {:?}", res);
    let res = counter.query_renew_map("hellotestarchid9".to_owned());
    println!("Res {:?}", res);
    // let res = counter.query_renew_map("hellotestarchid7".to_owned());
    // println!("Res {:?}", res);
    // let res = counter.query_renew_map(domain_name.to_owned());
    // println!("Res {:?}", res);
    // let res = counter.query_renew_map("hellotestarchid4".to_owned());
    // println!("Res {:?}", res);


    // let res = counter.set_default("testdomainx6.arch".to_string())?;

    // let default_domain = counter.query_domain_default(chain.sender())?;
    // println!("Default domain {}", default_domain.domain_id);


    Ok(())
}
