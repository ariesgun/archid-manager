use anyhow::Ok;
use contract_callback::{
    msg::{GetCountResponse, InstantiateMsg, QueryMsg}, AppContract, AppExecuteMsgFns, AppQueryMsgFns, ContractError
};
use cw_orch::mock::cw_multi_test::{Contract, ContractWrapper};

// Use prelude to get all the necessary imports
use cw_orch::mock::cw_multi_test::Executor;
use cw_orch::prelude::*;

use cosmwasm_std::{coin, coins, Addr, Uint128};

// consts for testing
const USER: &str = "user";
const ADMIN: &str = "admin";


pub fn cw721_archid_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_archid::entry::execute,
        cw721_archid::entry::instantiate,
        cw721_archid::entry::query,
    );
    Box::new(contract)
}

pub fn archid_registry_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        archid_registry::contract::execute,
        archid_registry::contract::instantiate,
        archid_registry::contract::query,
    )
    .with_reply(archid_registry::contract::reply);
    Box::new(contract)
}

#[test]
fn mint_domain() -> anyhow::Result<()> {

    let mock = MockBech32::new("mock");

    let user = mock.addr_make(USER);
    let admin = mock.addr_make(ADMIN);
    let wallet = mock.addr_make("wallet");
    
    mock.add_balance(
        &user, 
        vec![Coin {
            denom: "aarch".to_string(),
            amount: Uint128::new(1_000_000_000_000_000_000)
        }])?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    let domain_name = "domain1".to_string();

    // No fund should return error
    let res = contract
        .call_as(&user)
        .mint_domain(domain_name.clone(), &[]);
    let expected_err = ContractError::Payment(cw_utils::PaymentError::NoFunds {});
    assert_eq!(
        res.unwrap_err().downcast::<ContractError>()?,
        expected_err
    );

    // Mint a domain with fund
    contract
        .call_as(&user)
        .mint_domain(
            domain_name.clone(),
            &[Coin {
                denom: "aarch".to_string(),
                amount: Uint128::new(1_000_000_000_000_000_000)
            }]
        )?;
    
    let res: cw721_updatable::OwnerOfResponse = mock.wasm_querier().smart_query(
        cw721_archid_addr.clone(), 
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::OwnerOf { 
            token_id: domain_name.clone() + ".arch", 
            include_expired: None
        }
    )?;
    assert_eq!(res.owner, user);
    println!("{:?}", res);

    let res : cw721_updatable::NftInfoResponse<archid_token::Metadata> =  mock.wasm_querier().smart_query(
        cw721_archid_addr,
        &cw721_archid::msg::QueryMsg::<archid_token::Extension>::NftInfo { 
            token_id: domain_name.clone() + ".arch" 
        }
    )?;
    println!("{:?}", res);
    
    let res: archid_registry::msg::ResolveRecordResponse = mock.wasm_querier().smart_query(
        archid_registry_addr.clone(),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: domain_name.clone() + ".arch" }
    )?;
    println!("{:?}", res);
    assert_eq!(res.address, Some(user.to_string()));

    let resolve_info: archid_registry::msg::ResolveAddressResponse = mock.wasm_querier().smart_query(
        archid_registry_addr,
        &archid_registry::msg::QueryMsg::ResolveAddress { address: user }
    )?;
    assert_eq!(resolve_info.names, Some(vec![domain_name + ".arch"]));

    Ok(())

}

#[test]
fn set_domain_default() -> anyhow::Result<()> {

    let mock = MockBech32::new("mock");
    let user1 = mock.addr_make("user1");
    let user2 = mock.addr_make("user2");

    mock.add_balance(
        &user1, 
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    mock.add_balance(
        &user2,
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    contract
        .call_as(&user1)
        .mint_domain(
            "domain1_user1".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    contract
        .call_as(&user1)
        .mint_domain(
            "domain2_user1".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    contract
        .call_as(&user2)
        .mint_domain(
            "domain2_user2".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    // Set domain not owned
    // Mint a domain with fund
    let res = contract
        .call_as(&user1)
        .set_default("domain2_user2.arch".to_string());
    assert_eq!(
        res.unwrap_err().downcast::<ContractError>()?,
        ContractError::Unauthorized {}
    );

    // Set domain OK
    let _ = contract
        .call_as(&user1)
        .set_default("domain2_user1.arch".to_string())?;
    let default_resp = contract.query_domain_default(user1)?;
    assert_eq!(default_resp.domain_id, "domain2_user1.arch");

    Ok(())
}

#[test]
fn renew_domain() -> anyhow::Result<()> {

    let mock = MockBech32::new("mock");
    let user1 = mock.addr_make("user1");

    println!("User 1 {}", user1);

    mock.add_balance(
        &user1, 
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    contract
        .call_as(&user1)
        .mint_domain(
            "domain1".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;


    let approve_msg: cw721_archid::ExecuteMsg<Option<Empty>, Empty> = cw721_archid::msg::ExecuteMsg::<Option<Empty>, Empty>::Approve {
        spender: contract.addr_str()?.to_string(),
        token_id: "domain1.arch".to_string(),
        expires: None
    };
    
    let _ = mock
        .call_as(&user1)
        .execute(
        &approve_msg,
        &[],
        &cw721_archid_addr,
    )?;

    let res: archid_registry::msg::ResolveRecordResponse = mock.wasm_querier().smart_query(
        archid_registry_addr.clone(),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "domain1".to_string() + ".arch" }
    )?;
    println!("{:?}", res);
    assert_eq!(res.address, Some(user1.to_string()));
    assert_eq!(res.expiration, 1602555819);

    contract
        .call_as(&user1)
        .renew_domain(
            "domain1".to_string(), 
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    let res: archid_registry::msg::ResolveRecordResponse = mock.wasm_querier().smart_query(
        archid_registry_addr.clone(),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "domain1".to_string() + ".arch" }
    )?;
    println!("{:?}", res);
    assert_eq!(res.address, Some(user1.to_string()));
    assert_eq!(res.expiration, 1633314219);

    let resolve_info: archid_registry::msg::ResolveAddressResponse = mock.wasm_querier().smart_query(
        archid_registry_addr,
        &archid_registry::msg::QueryMsg::ResolveAddress { address: user1 }
    )?;
    assert_eq!(resolve_info.names, Some(vec!["domain1.arch".to_string()]));

    Ok(())
}

#[test]
fn schedule_and_cancel_domain_renewal() -> anyhow::Result<()> {

    let mock = MockBech32::new("mock");
    let user1 = mock.addr_make("user1");
    let user2 = mock.addr_make("user2");

    mock.add_balance(
        &user1, 
        coins(3_000_000_000_000_000_000, "aarch".to_string()))?;
    mock.add_balance(
        &user2, 
        coins(3_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    contract
        .call_as(&user1)
        .mint_domain(
            "domain1".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;
    contract
        .call_as(&user2)
        .mint_domain(
            "domain2".to_string(),
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    // Approve NFTs
    let approve_msg: cw721_archid::ExecuteMsg<Option<Empty>, Empty> = cw721_archid::msg::ExecuteMsg::<Option<Empty>, Empty>::Approve {
            spender: contract.addr_str()?.to_string(),
            token_id: "domain1.arch".to_string(),
            expires: None
        };
    let _ = mock
        .call_as(&user1)
        .execute(
        &approve_msg,
        &[],
        &cw721_archid_addr,
    )?;

    let approve_msg: cw721_archid::ExecuteMsg<Option<Empty>, Empty> = cw721_archid::msg::ExecuteMsg::<Option<Empty>, Empty>::Approve {
            spender: contract.addr_str()?.to_string(),
            token_id: "domain2.arch".to_string(),
            expires: None
        };
    let _ = mock
        .call_as(&user2)
        .execute(
        &approve_msg,
        &[],
        &cw721_archid_addr,
    )?;

    // Schedule renew
    contract
        .call_as(&user1)
        .schedule_auto_renew(
            "domain1".to_string(),
            &[coin(1_150_000_000_000_000_000, "aarch".to_string())]
        )?;

    contract
        .call_as(&user2)
        .schedule_auto_renew(
            "domain2".to_string(),
            &[coin(1_150_000_000_000_000_000, "aarch".to_string())]
        )?;

    let res: archid_registry::msg::ResolveRecordResponse = mock.wasm_querier().smart_query(
        archid_registry_addr.clone(),
        &archid_registry::msg::QueryMsg::ResolveRecord { name: "domain1".to_string() + ".arch" }
    )?;
    assert_eq!(res.address, Some(user1.to_string()));
    assert_eq!(res.expiration, 1602555819);

    let resolve_info: contract_callback::msg::RenewMapResponse = mock.wasm_querier().smart_query(
        contract.address()?,
        &QueryMsg::QueryRenewMap { domain_name: "domain1".to_string() }
    )?;
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().domain_id, "domain1");
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().callback_height, 6120000);
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().block_idx, 53);

    let resolve_info: contract_callback::msg::RenewMapResponse = mock.wasm_querier().smart_query(
        contract.address()?,
        &QueryMsg::QueryRenewMap { domain_name: "domain2".to_string() }
    )?;
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().domain_id, "domain2");
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().callback_height, 6120000);
    assert_eq!(resolve_info.renew_info.as_ref().unwrap().block_idx, 53);

    let resolve_info: contract_callback::msg::RenewJobsMapResponse = mock.wasm_querier().smart_query(
        contract.address()?,
        &QueryMsg::QueryRenewJobsMap { block_id: 53 }
    )?;
    assert_eq!(resolve_info.renew_jobs.len(), 2);

    // Cancel auto-renewal
    contract
        .call_as(&user2)
        .cancel_auto_renew("domain2".to_string())?;
    let resolve_info: contract_callback::msg::RenewJobsMapResponse = mock.wasm_querier().smart_query(
        contract.address()?,
        &QueryMsg::QueryRenewJobsMap { block_id: 53 }
    )?;
    assert_eq!(resolve_info.renew_jobs.len(), 1);

    let balance = mock.balance(user2, Some("aarch".to_string()))?;
    assert_eq!(balance, coins(1_850_000_000_000_000_000, "aarch".to_string()));
    
    Ok(())
}

#[test]
fn deposit_unauthorized() -> anyhow::Result<()> {
    let mock = MockBech32::new("mock");
    let user1 = mock.addr_make("user1");
    
    mock.add_balance(
        &user1, 
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    let res = contract
        .call_as(&user1)
        .deposit(
            &[coin(1_000_000_000_000_000_000, "aarch".to_string())]
        );

    assert_eq!(true, res.is_err());

    Ok(())
}

#[test]
fn withdraw_unauthorized() -> anyhow::Result<()> {
    let mock = MockBech32::new("mock");
    let user1 = mock.addr_make("user1");
    
    mock.add_balance(
        &user1, 
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    let res = contract
        .call_as(&user1)
        .withdraw();

    assert_eq!(true, res.is_err());

    Ok(())
}

#[test]
fn deposit_withdraw_funds() -> anyhow::Result<()> {
    let mock = MockBech32::new("mock");
    let admin: Addr = mock.sender.clone();
    
    mock.add_balance(
        &admin, 
        coins(5_000_000_000_000_000_000, "aarch".to_string()))?;

    let (contract, cw721_archid_addr, archid_registry_addr)  = setup(mock.clone())?;

    let res = contract
        .call_as(&admin)
        .deposit(
            &[coin(2_000_000_000_000_000_000, "aarch".to_string())]
        )?;

    let balance = mock.balance(contract.addr_str()?, Some("aarch".to_string()))?;

    assert_eq!(balance, coins(2_000_000_000_000_000_000, "aarch".to_string()));

    let res = contract
        .call_as(&admin)
        .withdraw()?;

    let balance = mock.balance(contract.addr_str()?, Some("aarch".to_string()))?;

    assert_eq!(balance, coins(0, "aarch".to_string()));

    Ok(())
}

/// Instantiate the contract in any CosmWasm environment
fn setup(mock: MockBech32) -> anyhow::Result<(AppContract<MockBech32>, Addr, Addr)> {
    // Construct the counter interface
    let contract = AppContract::new(mock.clone());
    let admin = mock.addr_make(ADMIN);
    let wallet = mock.addr_make("wallet");
    let empty = mock.addr_make("empty");

    println!("admin {}", admin);
    println!("wallet {}", wallet);
    println!("empty {}", empty);
    println!("mock sender {}", mock.sender());

    let cw721_archid_addr;
    let archid_registry_addr;

    {
        let mut app = mock.app.borrow_mut();
        
        let archid_registry_id = app.store_code(archid_registry_contract());
        archid_registry_addr = app.instantiate_contract(
            archid_registry_id,
            mock.sender().clone(),
            &archid_registry::msg::InstantiateMsg {
                admin: mock.sender.clone(),
                wallet: wallet.clone(),
                cw721: empty.clone(),
                base_cost: Uint128::new(1_000_000_000_000_000_000),
                base_expiration: 30758400,
            },
            &[],
            "archid-registry",
            None,
        )?;

        let cw721_archid_id = app.store_code(cw721_archid_contract());
        cw721_archid_addr = app.instantiate_contract(
            cw721_archid_id, 
            mock.sender.clone(), 
            &cw721_archid::msg::InstantiateMsg {
                name: "cw721_archid".to_string(),
                symbol: "ARCHID".to_string(),
                minter: archid_registry_addr.to_string(),
            },
            &[], 
            "cw721_archid", 
            None
        )?;

        app.execute_contract(
            mock.sender.clone(), 
            archid_registry_addr.clone(), 
            &archid_registry::msg::ExecuteMsg::UpdateConfig { 
                config: archid_registry::state::Config {
                    admin: admin.clone(),
                    wallet: wallet.clone(),
                    cw721: cw721_archid_addr.clone(),
                    base_cost: Uint128::new(1_000_000_000_000_000_000),
                    base_expiration: 30758400,
                }
            }, 
            &[]
        )?;

        println!("CW721: {:?} and archid_registry:{:?}", 
            cw721_archid_addr.clone(), 
            archid_registry_addr.clone()
        );
    }   

    // Upload the contract
    let upload_resp = contract.upload()?;

    // Get the code-id from the response.
    let code_id = upload_resp.uploaded_code_id()?;
    // or get it from the interface.
    assert_eq!(code_id, contract.code_id()?);

    // Instantiate the contract
    let msg = InstantiateMsg { 
        count: 1u64 ,
        cw721_archid_addr: cw721_archid_addr.clone(),
        archid_registry_addr: archid_registry_addr.clone(),
        denom: "aarch".to_string(),
        cost_per_year: 1_000_000_000_000_000_000_u128,
    };
    let init_resp = contract.instantiate(
        &msg, 
        Some(&admin), 
        None
    )?;

    // Get the address from the response
    let contract_addr = init_resp.instantiated_contract_address()?;
    // or get it from the interface.
    assert_eq!(contract_addr, contract.address()?);

    // Return the interface
    Ok((contract, cw721_archid_addr, archid_registry_addr))
}