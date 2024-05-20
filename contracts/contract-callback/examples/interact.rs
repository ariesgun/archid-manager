use archid_registry::state::Config;
use contract_callback::{msg::InstantiateMsg, AppContract, AppExecuteMsgFns, AppQueryMsgFns};
use cosmwasm_std::{to_json_binary, Uint128};
use cosmwasm_storage::ReadonlySingleton;
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

    let counter = AppContract::new(chain.clone());

    // Incrementing count
    println!("ID {:?}", counter.code_id());
    // counter.increment();
    println!("Count {:?}", counter.get_count()?);

    let domain_name = "testdomainx2";

    // let res = counter.mint_domain(
    //     domain_name.to_string(),
    //     &[Coin {
    //         amount: Uint128::new(1_000_000_000_000_000_000),
    //         denom: "aconst".to_string()
    //     }]
    // )?;
    // println!("Res {:?}", res);

    // Register a callback every day
    // let res: cw721_updatable::ContractInfoResponse = chain.wasm_querier().smart_query(
    //     &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
    //     &cw721_archid::msg::QueryMsg::<Empty>::ContractInfo {  }
    // )?;
    // println!("{:?}", res);

    let nft_id = domain_name.to_string() + ".arch";

    let approve_msg: cw721_archid::ExecuteMsg<Option<Empty>, Empty> = cw721_archid::msg::ExecuteMsg::<Option<Empty>, Empty>::Approve {
        spender: counter.addr_str()?.to_string(),
        token_id: nft_id.clone(),
        expires: None
    };

    // let res = chain.execute(
    //     &approve_msg,
    //     &[],
    //     &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
    // )?;
    // println!("{:?}", res);

    let res : cw721_updatable::NftInfoResponse<archid_token::Metadata> = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::NftInfo { token_id: nft_id.clone() }
    )?;
    println!("{:?}", res);

    let res : cw721_updatable::NftInfoResponse<archid_token::Metadata> = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::NftInfo { token_id: nft_id.clone() }
    )?;
    println!("{:?}", res);

    let res : cw721_updatable::OwnerOfResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway146htsfvftmq8fl26977w9xgdwmsptr2quuf7yyra4j0gttx32z3secq008"),
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::OwnerOf { token_id: nft_id.clone(), include_expired: None }
    )?;
    println!("{:?}", res);

    let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: nft_id.clone() }
    )?;
    println!("{:?}", res);

    let res: Config = chain.wasm_querier().smart_query(
        &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
        &archid_registry::msg::QueryMsg::Config {}
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

    // let res: archid_registry::msg::ResolveRecordResponse = chain.wasm_querier().smart_query(
    //     &Addr::unchecked("archway1lr8rstt40s697hqpedv2nvt27f4cuccqwvly9gnvuszxmcevrlns60xw4r"),
    //     &archid_registry::msg::QueryMsg::ResolveRecord { name: nft_id.clone() }
    // )?;
    // println!("{:?}", res);

    Ok(())
}
