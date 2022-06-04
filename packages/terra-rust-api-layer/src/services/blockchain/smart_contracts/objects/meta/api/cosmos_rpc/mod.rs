use tonic::transport::channel::Channel;

use std::time::Duration;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;

use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::SimulateRequest;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{Tx, TxBody};

use cosmos_sdk_proto::cosmwasm::wasm::v1::query_client::QueryClient;
use cosmos_sdk_proto::cosmwasm::wasm::v1::QueryContractInfoRequest;
use cosmrs::tendermint::block;
use cosmrs::Coin;
use cosmrs::bank::MsgSend;
use cosmrs::tx::Fee;
use cosmrs::tx::SignerInfo;
use cosmrs::tx::SignDoc;
use cosmrs::tx::{MsgProto, Msg};
use cosmrs::tx::AccountNumber;
use prost_types::Any;

use osmosis_proto::custom_cosmrs::{MsgProto as OsmosisMsgProto, Msg as OsmosisMsg};

use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgExecuteContract;
use osmosis_proto::osmosis::gamm::v1beta1::query_client::QueryClient as OsmosisQueryClient;
use osmosis_proto::osmosis::gamm::v1beta1::{QueryNumPoolsRequest, QueryNumPoolsResponse, QueryPoolsRequest, QueryPoolsResponse, Pool, QueryPoolRequest, QueryPoolResponse};

use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient as AuthQueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest, QueryAccountResponse};
use cosmos_sdk_proto::cosmos::vesting::v1beta1::{PeriodicVestingAccount};

use cosmrs::tx::Body;

use moneymarket::market::ExecuteMsg;

use osmo_bindings::OsmosisQuery::PoolState;

/*
/// Chain ID to use for tests
//const CHAIN_ID: &str = "pisco-1";
const CHAIN_ID: &str = "phoenix-1";
/// Expected account number
const ACCOUNT_NUMBER: AccountNumber = 1;
/// Bech32 prefix for an account
const ACCOUNT_PREFIX: &str = "terra";
/// Denom name
const DENOM: &str = "uluna";
/// Example memo
const MEMO: &str = "test memo";
*/

pub async fn get_osmosis_channel() -> anyhow::Result<Channel> {
    let channel = Channel::from_static("http://46.38.251.100:9090") // Felix | Interbloc
        //let channel = Channel::from_static("http://v-terra-hel-1.zyons.com:29090")
        //let channel = Channel::from_static("http://osmosis.strange.love:9090")
        //let channel = Channel::from_static("http://cosmoshub.validator.network:443")
        //let channel = Channel::from_static("http://cosmos.chorus.one:26657")
        //let channel = Channel::from_static("http://rpc.cosmos.network:26657")
        //let channel = Channel::from_static("http://a.client.sentry.neerajnet.bluzelle.com:9090")
        //let channel = Channel::from_static("http://grpc-osmosis-ia.notional.ventures:443")
        .timeout(Duration::from_secs(5))
        .rate_limit(5, Duration::from_secs(1))
        .concurrency_limit(256)
        .connect()
        .await?;
    Ok(channel)
}

pub async fn get_terra_channel() -> anyhow::Result<Channel> {
    //let channel = Channel::from_static("http://v-terra-hel-1.zyons.com:29090")
    let channel = Channel::from_static("http://n-fsn-7.zyons.com:29090")
        .timeout(Duration::from_secs(5))
        .rate_limit(5, Duration::from_secs(1))
        .concurrency_limit(256)
        .connect()
        .await?;
    Ok(channel)
}

pub async fn get_pool_count() -> anyhow::Result<QueryNumPoolsResponse> {
    let channel = get_osmosis_channel().await?;
    let res = OsmosisQueryClient::new(channel).num_pools(QueryNumPoolsRequest {}).await?.into_inner();
    //println!("{:?}", &res.num_pools);
    Ok(res)
}

pub async fn get_pools_info() -> anyhow::Result<Vec<Pool>> {
    let channel = get_osmosis_channel().await?;
    let res = OsmosisQueryClient::new(channel).pools(QueryPoolsRequest {
        pagination: Some(PageRequest {
            key: vec![],
            offset: 0,
            limit: 100,
            count_total: false,
            reverse: false,
        })
    }).await?.into_inner();

    let pools: Vec<Pool> = res.pools.into_iter().map(|x| OsmosisMsgProto::from_any(&x).unwrap()).collect();
    //println!("{:?}", pools);
    Ok(pools)
}

pub async fn get_pool_info(pool_id: u64) -> anyhow::Result<Pool> {
    let channel = get_osmosis_channel().await?;
    let res = OsmosisQueryClient::new(channel).pool(QueryPoolRequest {
        pool_id: pool_id,
    }).await?.into_inner();

    let pool: Pool = OsmosisMsgProto::from_any(&res.pool.unwrap()).unwrap();
    //println!("{:?}", pool);
    Ok(pool)
}

pub async fn msg_send() -> anyhow::Result<()> {
    let channel = get_terra_channel().await?;

    /*
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(&body, &auth_info, &chain_id, ACCOUNT_NUMBER).unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();
    */

    /*
    let res = QueryClient::new(channel).contract_info(QueryContractInfoRequest { address: "terra".to_string() }).await?.into_inner();
    println!("{:?}", (res.address, res.contract_info.as_ref().unwrap().label.as_str()));

    let res = QueryClient::new(channel).contract_info(QueryContractInfoRequest { address: "terra".to_string() }).await?.into_inner();
    println!("{:?}", (res.address, res.contract_info.as_ref().unwrap().label.as_str()));
    */
    let res: QueryAccountResponse = AuthQueryClient::new(channel).account(QueryAccountRequest { address: "terra".to_string() }).await?.into_inner();
    println!("{:?}", res.account.as_ref().unwrap().value);
    println!("{:?}", res.account.as_ref().unwrap().type_url);
    let periodic_vesting_account: PeriodicVestingAccount = MsgProto::from_any(&res.account.as_ref().unwrap()).unwrap();
    println!("{:?}", periodic_vesting_account);

    Ok(())
}

/* // Example for MsgExecuteContract
pub async fn msg_send() -> anyhow::Result<()> {

    let channel = get_terra_channel().await?;

    let contract_addr_mm_market = "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal".to_string();

    let execute_msg = ExecuteMsg::ClaimRewards { to: None };
    let execute_msg_json = serde_json::to_string(&execute_msg)?;

    let msg_execute_contract_proto = MsgExecuteContract {
        sender: "terra".to_string(),
        contract: contract_addr_mm_market,
        msg: execute_msg_json.as_bytes().to_vec(),
        funds: vec![],
    };
    let msg_execute = MsgProto::to_any(&msg_execute_contract_proto).unwrap()/*.to_any().unwrap()*/;

    let body = TxBody {
        messages: vec![msg_execute.into()],
        memo: MEMO.to_string(),
        timeout_height: 100000u64,
        extension_options: vec![],
        non_critical_extension_options: vec![],
    };

    let transaction = Tx {
        body: Some(body/*.into_proto()*/),
        auth_info: None,
        signatures: vec![],
    };
    let res = ServiceClient::new(channel).simulate(SimulateRequest {
        tx: None, // deprecated
        tx_bytes: prost::Message::encode_to_vec(&transaction),
    }).await?.into_inner();

    println!("{:?}", res);

    Ok(())
}*/



/* // Example to use tendermint_rpc
use cosmrs::{
    query,
    bank::MsgSend,
    crypto::secp256k1,
    dev, rpc,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};
use std::{panic, str};
use cosmrs::rpc::query::Query;
use cosmrs::rpc::{Client, query_client};

use terra_cosmwasm::{TerraQuerier, SwapResponse, TaxRateResponse, TaxCapResponse, ExchangeRatesResponse};

/// RPC port
const RPC_PORT: u16 = 26657;

pub async fn msg_send() {
    let rpc_address = format!("http://v-terra-hel-1.zyons.com:{}", RPC_PORT);
    //let rpc_address = format!("http://n-fsn-7.zyons.com:{}", RPC_PORT);

    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    println!("rpc_client loaded");
    println!("{:?}", rpc_client.latest_block().await);
    // https://docs.cosmos.network/master/core/grpc_rest.html
    rpc_client.abci_query()

    ::default();
}
*/