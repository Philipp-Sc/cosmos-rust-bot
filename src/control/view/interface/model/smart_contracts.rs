// https://lcd.terra.dev/swagger/#/

pub mod meta; 

use serde::Deserialize;
use serde::Serialize;

use serde_json::Value; 

//use rust_decimal::Decimal;
use core::str::FromStr;
  

use cosmwasm_std::{Uint128,Uint256,Decimal256,Decimal};

use meta::api::data::{GasPrices};

use meta::api::data::terra_contracts::{get_contract,get_mirrorprotocol_assets};

use meta::api::{
    get_fcd_or_lcd_query,
   /* get_fcd_else_lcd_query,
    get_lcd_else_fcd_query,
    get_fcd_query,
    get_lcd_query,*/
    query_core_market_swap_rate,
    query_core_bank_balances,
    query_api,
    query_core_latest_block,
    query_core_block_at_height,
    query_core_block_txs};
 

use anyhow::anyhow;
use enum_as_inner::EnumAsInner;


//use terra_rust_api::client::tx_types::V1TXSResult;

use regex::Regex;

use chrono::{DateTime};


use std::time::{Instant};


#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]  
pub enum ResponseResult {
    Text(String),
    State(StateResponse),
    EpochState(EpochStateResponse),
    Config(ConfigResponse),
    Simulation(Response<SimulationResponse>),
    CoreSwap(Response<CoreSwapResponse>),
    Price(Response<PriceResponse>),
    BorrowLimit(Response<BorrowLimitResponse>),
    BorrowInfo(Response<BorrowInfoResponse>),
    Balance(Response<BalanceResponse>),
    Balances(Response<Vec<Coin>>),
    Staker(Response<StakerResponse>), 
    DistributionApy(DistributionApyResponse),
    GovReward(GovRewardResponse),
    Blocks(Response<BlocksPerYearResponse>),
    StablecoinDeposits(Response<Vec<DepositStableLog>>),
    Transactions(Response<Vec<TXLog>>),
    EarnAPY(Response<APY>),
    TaxRate(Response<String>),
    TaxCaps(Response<Vec<TaxCap>>)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}


#[derive(Serialize, Deserialize,Clone, Debug,PartialEq)]
pub struct APY {
    pub apy: rust_decimal::Decimal,
    pub result: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlocksPerYearResponse {
    pub blocks_per_year: f64,
    pub blocks_per_millis: f64,
    pub latest_block: String, 
    pub historic_block: String,  
}

#[derive(Serialize, Deserialize,Clone, Debug,PartialEq)]
pub struct Response<T> {
    pub height: String,
    pub result: T
}  

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BLunaStateResponse {
    pub bluna_exchange_rate: Decimal,
    pub stluna_exchange_rate: Decimal,
    pub total_bond_bluna_amount: Uint128,
    pub total_bond_stluna_amount: Uint128,
    pub last_index_modification: u64,
    pub prev_hub_balance: Uint128,
    pub last_unbonded_time: u64,
    pub last_processed_batch: u64,
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MarketStateResponse {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/market#stateresponse
    pub total_liabilities: Decimal256, 
    pub total_reserves: Decimal256, 
    pub last_interest_updated: u64, 
    pub last_reward_updated: u64, 
    pub global_interest_index: Decimal256, 
    pub global_reward_index: Decimal256, 
    pub anc_emission_rate: Decimal256, 
    pub prev_aterra_supply: Uint256, 
    pub prev_exchange_rate: Decimal256, 
}


#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)]  
pub enum StateResponse {
    #[allow(non_camel_case_types)]
    bLunaHub(Response<BLunaStateResponse>),
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketStateResponse>), 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MarketEpochStateResponse {
    pub exchange_rate: Decimal256, 
    pub aterra_supply: Uint256, 
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)] 
pub enum EpochStateResponse { 
    #[allow(non_camel_case_types)]
    mmMarket(Response<MarketEpochStateResponse>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InterestModelConfigResponse {
    pub owner: String, 
    pub base_rate: Decimal256, 
    pub interest_multiplier: Decimal256, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CollectorConfigResponse {
    pub gov_contract: String, 
    pub terraswap_factory: String,
    pub anchor_token: String,
    pub distributor_contract: String,
    pub reward_factor: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumAsInner)] 
pub enum ConfigResponse { 
    #[allow(non_camel_case_types)]
    mmInterestModel(Response<InterestModelConfigResponse>),
    Collector(Response<CollectorConfigResponse>),
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SimulationResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct CoreSwapResponse {
    pub amount: Uint128,
    pub denom: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PriceResponse {
    pub rate: Decimal,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowLimitResponse {
    pub borrower: String, 
    pub borrow_limit: Uint128, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowInfoResponse {
    pub borrower: String, 
    pub interest_index: Decimal256, 
    pub reward_index: Decimal256, 
    pub loan_amount: Uint256, 
    pub pending_rewards: Decimal256, 
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BalanceResponse {
    pub balance: Uint128,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StakerResponse {
    pub balance: Uint128,
    pub share: Uint128,
    pub locked_balance: Vec<(u64, VoterInfo)>, // (Voted Poll's ID, VoterInfo)
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VoterInfo {
    pub vote: VoteOption,
    pub balance: Uint128,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoteOption {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DistributionApyResponse {
    pub anc_price: Decimal,
    pub height: u64,
    pub timestamp: u64,
    pub anc_emission_rate: Decimal,
    pub total_liabilities: Decimal,
    pub distribution_apy: Decimal, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GovRewardResponse {
    pub height: u64,
    pub timestamp: u64,
    pub gov_share_index: Decimal,
    pub current_apy: Decimal,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxCap {
    pub denom: String,
    #[serde(rename = "tax_cap")]
    pub tax_cap: String,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    pub logs: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(rename = "msg_index")]
    pub msg_index: i64,
    pub log: String,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DepositStableLog { 
    pub height: u64,
    pub timestamp: i64, //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
    pub mint_amount: rust_decimal::Decimal,
    pub deposit_amount: rust_decimal::Decimal,
    pub exchange_rate: rust_decimal::Decimal, 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TXLog { 
    pub height: u64,
    pub timestamp: i64, //chrono::datetime::DateTime<chrono::offset::utc::Utc> .timestamp()
    pub gas_wanted: rust_decimal::Decimal,
    pub gas_used: rust_decimal::Decimal,
    pub fee_denom: String,
    pub fee_amount: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
    pub raw_log: String, 
}
 

pub async fn get_block_txs_deposit_stable_apy() -> anyhow::Result<ResponseResult> { 

    let latest_block = query_core_latest_block().await?;      
    let historic_block_height = latest_block.block.header.height - (60*60*24*30)/6;
    let historic_block = query_core_block_at_height(historic_block_height as u64).await?;

    let latest_stablecoin_deposits = get_block_txs_deposit_stable(latest_block.block.header.height).await?;
    let historic_stablecoin_deposits = get_block_txs_deposit_stable(historic_block.block.header.height).await?;

    let latest_result = &latest_stablecoin_deposits.as_stablecoin_deposits().unwrap().result[0];
    let historic_result = &historic_stablecoin_deposits.as_stablecoin_deposits().unwrap().result[0];

    let exchange_rate_difference = latest_result.exchange_rate.checked_sub(historic_result.exchange_rate).unwrap();
    let time_difference: i64 = latest_result.timestamp - historic_result.timestamp;
    let unit: f64 = (365*24*60*60) as f64 / time_difference as f64;

    let exchange_rate_difference_anual = rust_decimal::Decimal::from_str(unit.to_string().as_str()).unwrap().checked_mul(exchange_rate_difference).unwrap();

     Ok(ResponseResult::EarnAPY(Response{
        height: latest_block.block.header.height.to_string(), 
        result: APY {
            apy: exchange_rate_difference_anual.checked_div(latest_result.exchange_rate).unwrap(),
            result: format!("{:?}\n\n{:?}",latest_stablecoin_deposits.as_stablecoin_deposits().unwrap().result,historic_stablecoin_deposits.as_stablecoin_deposits().unwrap().result)
        }
        }))
}

// next todo: list last 10, claim_rewards gas fees.
/*
    
    // prüfe query

  "body": {
          "messages": [
            {
              "@type": "/terra.wasm.v1beta1.MsgExecuteContract",
              "sender": "***REMOVED***",
              "contract": "terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s", // mmMarket
              "execute_msg": {"claim_rewards":{}},                        // claim_rewards (ANC)
              "coins": [
              ]
            }
          ],

    // prüfe resultat

      {
              "type": "wasm",
              "attributes": [
                {
                  "key": "contract_address",
                  "value": "terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s"
                },
                {
                  "key": "action",
                  "value": "claim_rewards"
                },
                {
                  "key": "claim_amount",
                  "value": "9896440"
                },
                {
                  "key": "contract_address",
                  "value": "terra1mxf7d5updqxfgvchd7lv6575ehhm8qfdttuqzz"
                },
                {
                  "key": "action",
                  "value": "spend"
                },
                {
                  "key": "recipient",
                  "value": "***REMOVED***"
                },
                {
                  "key": "amount",
                  "value": "9896440"
                },
                {
                  "key": "contract_address",
                  "value": "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76"
                },
                {
                  "key": "action",
                  "value": "transfer"
                },
                {
                  "key": "from",
                  "value": "terra1mxf7d5updqxfgvchd7lv6575ehhm8qfdttuqzz"
                },
                {
                  "key": "to",
                  "value": "***REMOVED***"
                },
                {
                  "key": "amount",
                  "value": "9896440"
                }
              ]


    // verwende

    "fee": {
            "amount": [
              {
                "denom": "uusd",
                "amount": "250657"
              }
            ],
            "gas_limit": "1000000",
            "payer": "",
            "granter": ""
          }


      "gas_wanted": "1000000",
      "gas_used": "261026", 

*/

// https://lcd.terra.dev/swagger/#/Service/Simulate
// I can also simulate first.

 
/*
pub async fn get_block_txs_staking() -> anyhow::Result<ResponseResult> { 

    // https://fcd.terra.dev/cosmos/tx/v1beta1/txs?events=tx.height=5757663&order_by=ORDER_BY_ASC&pagination.limit=10000000&pagination.offset=0

    let latest_block = query_core_latest_block().await?;      
    let height = latest_block.block.header.height;
    
    let mut staking: Vec<TXLog> = Vec::new();
    let mut temp_height = height;

    while staking.len()<5 {

        //println!("{:?}",temp_height);

        let transactions = query_core_block_txs(temp_height,None, Some(200)).await?; // terra_rust_api::client::tx_types::V1TXSResult
        let tx_responses = transactions.tx_responses;

        for index in 0..tx_responses.len() { 

            if tx_responses[index].raw_log.contains("staking") 
                && tx_responses[index].raw_log.contains("amount") 
                && tx_responses[index].raw_log.contains("terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5") 
                && tx_responses[index].raw_log.contains("terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76")  {


                let messages = tx_responses[index].tx.get("body").unwrap().get("messages").unwrap();
                let fee = tx_responses[index].tx.get("auth_info").unwrap().get("fee").unwrap().get("amount").unwrap();

                if messages.as_array().unwrap().len()==1 {
                    if messages[0].get("contract").unwrap()=="terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76"
                        &&  messages[0].get("execute_msg").unwrap().get("send").unwrap().get("msg").unwrap() == "eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0=" { // {"stake_voting_tokens":{}}

                        // https://docs.cosmos.network/master/basics/tx-lifecycle.html

                        // --gas refers to how much gas, which represents computational resources, Tx consumes. 
                        // Gas is dependent on the transaction and is not precisely calculated until execution.

                        let gas_wanted = tx_responses[index].gas_wanted;  // gas_limit // gas requested
                        let gas_used = tx_responses[index].gas_used;      // used

                        // The user-provided amount of gas for Tx is known as GasWanted. 
                        // If GasConsumed, the amount of gas consumed so during execution, ever exceeds GasWanted, the execution will stop and the changes made to the cached copy of the state won't be committed. 
                        // Otherwise, CheckTx sets GasUsed equal to GasConsumed and returns it in the result.

                        // https://docs.cosmos.network/master/basics/gas-fees.html
                        // As explained above, the anteHandler returns a maximum limit of gas the transaction can consume during execution called GasWanted. 
                        // The actual amount consumed in the end is denominated GasUsed, and we must therefore have GasUsed =< GasWanted


                        // gas-prices specifies how much the user is willing pay per unit of gas, which can be one or multiple denominations of tokens. For example, --gas-prices=0.025uatom, 0.025upho means the user is willing to pay 0.025uatom AND 0.025upho per unit of gas.
                        // fee = gas_price * gas_used
                        // fee = (gas_used * gas_adjustment) * gas_price + tax (0 in this case) 


                        let fist_tx_log: Log = serde_json::from_str(tx_responses[index].logs[0].to_string().as_str())?;
                
                        let mut stake_amount = "--".to_string();
                        for i in 0..fist_tx_log.events.len() {
                            if fist_tx_log.events[i].type_field == "wasm" {
                                for ii in 0..fist_tx_log.events[i].attributes.len() {
                                    if fist_tx_log.events[i].attributes[ii].key == "amount" {
                                        stake_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();
                                    } 
                                }
                            }
                        }  

                        if fee.as_array().unwrap().len() == 1 { 

                            if fee[0].get("denom").unwrap().to_string().contains("uusd") {

                                let re = Regex::new(r"[^0-9]").unwrap();

                                staking.push( TXLog { 
                                    height: temp_height, 
                                    timestamp: tx_responses[index].timestamp.timestamp(), 
                                    gas_wanted: rust_decimal::Decimal::from_str(gas_wanted.to_string().as_str()).unwrap(), 
                                    gas_used: rust_decimal::Decimal::from_str(gas_used.to_string().as_str()).unwrap(), 
                                    amount: rust_decimal::Decimal::from_str(stake_amount.as_str()).unwrap(), 
                                    fee_denom: "uusd".to_string(),
                                    fee_amount: rust_decimal::Decimal::from_str(re.replace_all(fee[0].get("amount").unwrap().to_string().as_str(), "").to_string().as_str()).unwrap(),
                                    // gas_adjustment = fee_amount / (gas_wanted * gas_price)
                                    raw_log: tx_responses[index].raw_log.to_owned()});
                            }
               
                        } 
                        
                    }
                }
 
 
            }
        }
        temp_height = temp_height - 1;
    }

    Ok(ResponseResult::Transactions(Response{
        height: height.to_string(),
        result: staking
        }))
}
*/

fn get_tx_log(entry: &Value, account: &str, query_msg: &str, amount_field: &str) -> anyhow::Result<TXLog> {

    let msg = entry.get("tx").ok_or(anyhow!("no tx"))?.get("value").ok_or(anyhow!("no value"))?.get("msg").ok_or(anyhow!("no msg"))?.as_array().ok_or(anyhow!("no array"))?; 
               
    if  msg.len() == 1 &&
        (   (query_msg=="staking" && 
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("msg").ok_or(anyhow!("no msg"))?.to_string().contains("eyJzdGFrZV92b3RpbmdfdG9rZW5zIjp7fX0=")
            ) 
        || (query_msg=="claim_rewards" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("claim_rewards") != None
            )
        || (query_msg=="redeem_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("send").ok_or(anyhow!("no send"))?.get("msg").ok_or(anyhow!("no msg"))?.to_string().contains("eyJyZWRlZW1fc3RhYmxlIjp7fX0=")
            )
        || (query_msg=="deposit_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("deposit_stable") != None
            )
        || (query_msg=="repay_stable" &&
            msg[0].get("value").ok_or(anyhow!("no value"))?.get("execute_msg").ok_or(anyhow!("no execute_msg"))?.get("repay_stable") != None
            )
        ) && msg[0].get("value").ok_or(anyhow!("no value"))?.get("contract").ok_or(anyhow!("no contract"))? == account
    {
            let gas_wanted = entry.get("gas_wanted").ok_or(anyhow!("no gas_wanted"))?;  // gas_limit // gas requested
            let gas_used = entry.get("gas_used").ok_or(anyhow!("no gas_used"))?;        // used


            let events = entry.get("logs").ok_or(anyhow!("no logs"))?.as_array().ok_or(anyhow!("no array"))?;
            if events.len() > 0 {

                let events = events[0].get("events").ok_or(anyhow!("no events"))?.as_array().ok_or(anyhow!("no array"))?;


                let mut amount = "0".to_string();
                for i in 0..events.len() {
                    if events[i].get("type").ok_or(anyhow!("no type"))? == "wasm" {
                        for ii in 0..events[i].get("attributes").ok_or(anyhow!("no attributes"))?.as_array().ok_or(anyhow!("no array"))?.len() {
                            if events[i].get("attributes").unwrap().as_array().unwrap()[ii].get("key").ok_or(anyhow!("no key"))?.to_string().contains(amount_field) {
                                amount = events[i].get("attributes").unwrap().as_array().unwrap()[ii].get("value").ok_or(anyhow!("no value"))?.to_string();
                            } 
                        }
                    }
                }


                let fee = entry.get("tx").unwrap().get("value").unwrap().get("fee").ok_or(anyhow!("no fee"))?;
                //let gas_limit = fee.get("gas").ok_or(anyhow!("no gas"))?; // same as gas_wanted
                
                let fee = fee.get("amount").ok_or(anyhow!("no amount"))?.as_array().ok_or(anyhow!("no array"))?;

                if fee.len() == 1 {

                    if fee[0].get("denom").ok_or(anyhow!("no denom"))?.to_string().contains("uusd") {

                       
                        let re = Regex::new(r"[^0-9]").unwrap();

                        let transaction_fee = re.replace_all(fee[0].get("amount").ok_or(anyhow!("no amount"))?.to_string().as_str(), "").to_string();
                        let tx_height = re.replace_all(entry.get("height").ok_or(anyhow!("no height"))?.to_string().as_str(), "").to_string();

                        let gas_used = re.replace_all(gas_used.to_string().as_str(), "").to_string();
                        let gas_wanted =  re.replace_all(gas_wanted.to_string().as_str(), "").to_string();

                        let amount = re.replace_all(amount.as_str(), "").to_string();


                        return Ok(TXLog { 
                            height: tx_height.parse::<u64>()?, 
                            timestamp: DateTime::parse_from_rfc3339(entry.get("timestamp").ok_or(anyhow!("no timestamp"))?.to_string().replace(r#"""#, "").as_str())?.timestamp(), 
                            gas_wanted: rust_decimal::Decimal::from_str(gas_wanted.as_str())?, 
                            gas_used: rust_decimal::Decimal::from_str(gas_used.as_str())?, 
                            amount: rust_decimal::Decimal::from_str(amount.as_str())?, 
                            fee_denom: "uusd".to_string(),
                            fee_amount: rust_decimal::Decimal::from_str(transaction_fee.as_str())?,
                            // gas_adjustment = fee_amount / (gas_wanted * gas_price)
                            raw_log: entry.get("raw_log").ok_or(anyhow!("no raw_log"))?.to_string()
                        });
                    }
                }
            }
    }
    Err(anyhow!("Error: Invalid format."))
}


pub async fn get_tax_rate() -> anyhow::Result<ResponseResult> { 
        let res: String = query_api("https://lcd.terra.dev/treasury/tax_rate").await?;
        let res: Response<String> = serde_json::from_str(&res)?;
        return Ok(ResponseResult::TaxRate(res));
}

pub async fn get_tax_caps() -> anyhow::Result<ResponseResult> { 
        let res: String = query_api("https://lcd.terra.dev/treasury/tax_caps").await?;
        let res: Response<Vec<TaxCap>> = serde_json::from_str(&res)?;
        return Ok(ResponseResult::TaxCaps(res));
}

pub async fn get_txs_fee_data(offset: &str, tx_data: &mut Vec<TXLog>,account: &str, query_msg: &str, amount_field: &str) -> anyhow::Result<String> {

        let query = format!("https://fcd.terra.dev/v1/txs?offset={}&limit=100&account={}",offset, account); 
        let res: String = query_api(query.as_str()).await?;
        let res: Value = serde_json::from_str(&res)?;

        let entries = res.get("txs").ok_or(anyhow!("no txs"))?.as_array().ok_or(anyhow!("no array"))?;
 
        for entry in entries {
             match get_tx_log(entry, account, query_msg, amount_field) {
                Ok(txlog) => {
                    tx_data.push(txlog);
                },
                Err(_) => {
                }
             };
        } 
        let re = Regex::new(r"[^0-9]").unwrap();
        Ok(re.replace_all(res.get("next").ok_or(anyhow!("no next"))?.to_string().as_str(),"").to_string())
}


pub async fn get_block_txs_fee_data(key: &str) -> anyhow::Result<ResponseResult> { 
 
    let start = Instant::now();  

    let mut tx_data: Vec<TXLog> = Vec::new();
    let mut temp_offset = "0".to_string(); 

    let mut err_count = 0;

    while tx_data.len()<10 && start.elapsed().as_secs() < 60*3 && err_count < 2 {
        let mut next: anyhow::Result<String> = Ok("0".to_string());
        if key == "claim_rewards" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,"terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s","claim_rewards","claim_amount").await;
        }
        if key == "staking" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,"terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76","staking","amount").await; 
        }
        if key == "redeem_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,"terra1hzh9vpxhsk8253se0vv5jj6etdvxu3nv8z07zu","redeem_stable","redeem_amount").await; 
        }
        if key == "deposit_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,"terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s","deposit_stable","deposit_amount").await; 
        }
        if key == "repay_stable" {
            next = get_txs_fee_data(temp_offset.as_str(),&mut tx_data,"terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s","repay_stable","repay_amount").await; 
        }

        if next.is_ok() {
            temp_offset = next.unwrap();
            err_count = 0;
        }else{
            err_count = err_count + 1;
        }
    }

    if tx_data.len()<10 && start.elapsed().as_secs() >= 60*3 {
        return Err(anyhow!("Unexpected Error: Timeout!"));
    }

    Ok(ResponseResult::Transactions(Response{
        height: tx_data[0].height.to_string(),
        result: tx_data
        }))

}

/*
pub async fn get_block_txs_claim_rewards_old() -> anyhow::Result<ResponseResult> { 

    // https://fcd.terra.dev/cosmos/tx/v1beta1/txs?events=tx.height=5757663&order_by=ORDER_BY_ASC&pagination.limit=10000000&pagination.offset=0

    let latest_block = query_core_latest_block().await?;      
    let height = latest_block.block.header.height;
    
    let mut claim_rewards: Vec<TXLog> = Vec::new();
    let mut temp_height = height;

    while claim_rewards.len()<5 {

        //println!("{:?}",temp_height);

        let transactions = query_core_block_txs(temp_height,None, Some(200)).await?; // terra_rust_api::client::tx_types::V1TXSResult
        let tx_responses = transactions.tx_responses;

        for index in 0..tx_responses.len() { 

            if tx_responses[index].raw_log.contains("claim_rewards") 
                && tx_responses[index].raw_log.contains("claim_amount") 
                && tx_responses[index].raw_log.contains("terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s") 
                && tx_responses[index].raw_log.contains("terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76") 
                && tx_responses[index].raw_log.contains("terra1mxf7d5updqxfgvchd7lv6575ehhm8qfdttuqzz") {


                let messages = tx_responses[index].tx.get("body").unwrap().get("messages").unwrap();
                let fee = tx_responses[index].tx.get("auth_info").unwrap().get("fee").unwrap().get("amount").unwrap();

                if messages.as_array().unwrap().len()==1 {
                    if messages[0].get("contract").unwrap()=="terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s"
                        &&  messages[0].get("execute_msg").unwrap().get("claim_rewards") != None {

                        // https://docs.cosmos.network/master/basics/tx-lifecycle.html

                        // --gas refers to how much gas, which represents computational resources, Tx consumes. 
                        // Gas is dependent on the transaction and is not precisely calculated until execution.

                        let gas_wanted = tx_responses[index].gas_wanted;  // gas_limit // gas requested
                        let gas_used = tx_responses[index].gas_used;      // used

                        // The user-provided amount of gas for Tx is known as GasWanted. 
                        // If GasConsumed, the amount of gas consumed so during execution, ever exceeds GasWanted, the execution will stop and the changes made to the cached copy of the state won't be committed. 
                        // Otherwise, CheckTx sets GasUsed equal to GasConsumed and returns it in the result.

                        // https://docs.cosmos.network/master/basics/gas-fees.html
                        // As explained above, the anteHandler returns a maximum limit of gas the transaction can consume during execution called GasWanted. 
                        // The actual amount consumed in the end is denominated GasUsed, and we must therefore have GasUsed =< GasWanted


                        // gas-prices specifies how much the user is willing pay per unit of gas, which can be one or multiple denominations of tokens. For example, --gas-prices=0.025uatom, 0.025upho means the user is willing to pay 0.025uatom AND 0.025upho per unit of gas.
                        // fee = gas_price * gas_used
                        // fee = (gas_used * gas_adjustment) * gas_price + tax (0 in this case) 


                        let fist_tx_log: Log = serde_json::from_str(tx_responses[index].logs[0].to_string().as_str())?;
                
                        let mut claim_amount = "--".to_string();
                        for i in 0..fist_tx_log.events.len() {
                            if fist_tx_log.events[i].type_field == "wasm" {
                                for ii in 0..fist_tx_log.events[i].attributes.len() {
                                    if fist_tx_log.events[i].attributes[ii].key == "claim_amount" {
                                        claim_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();
                                    } 
                                }
                            }
                        }  

                        if fee.as_array().unwrap().len() == 1 { 

                            if fee[0].get("denom").unwrap().to_string().contains("uusd") {

                                let re = Regex::new(r"[^0-9]").unwrap();

                                claim_rewards.push( TXLog { 
                                    height: temp_height, 
                                    timestamp: tx_responses[index].timestamp.timestamp(), 
                                    gas_wanted: rust_decimal::Decimal::from_str(gas_wanted.to_string().as_str()).unwrap(), 
                                    gas_used: rust_decimal::Decimal::from_str(gas_used.to_string().as_str()).unwrap(), 
                                    amount: rust_decimal::Decimal::from_str(claim_amount.as_str()).unwrap(), 
                                    fee_denom: "uusd".to_string(),
                                    fee_amount: rust_decimal::Decimal::from_str(re.replace_all(fee[0].get("amount").unwrap().to_string().as_str(), "").to_string().as_str()).unwrap(),
                                    // gas_adjustment = fee_amount / (gas_wanted * gas_price)
                                    raw_log: tx_responses[index].raw_log.to_owned()});
                            }
               
                        } 
                        
                    }
                }
 
 
            }
        }
        temp_height = temp_height - 1;
    }

    Ok(ResponseResult::Transactions(Response{
        height: height.to_string(),
        result: claim_rewards
        }))
}
*/

pub async fn get_block_txs_deposit_stable(height: u64) -> anyhow::Result<ResponseResult> { 


    let start = Instant::now();  

    let mut stablecoin_deposits: Vec<DepositStableLog> = Vec::new();
    let mut count = 0;

    while stablecoin_deposits.len()<1 && start.elapsed().as_secs()<60*3 {

        let transactions = query_core_block_txs(height-count,None, Some(100)).await?; // terra_rust_api::client::tx_types::V1TXSResult
        let tx_responses = transactions.tx_responses;

        for index in 0..tx_responses.len() {
            if tx_responses[index].raw_log.contains("deposit_stable") 
                && tx_responses[index].raw_log.contains("mint_amount") 
                && tx_responses[index].raw_log.contains("deposit_amount") {

                let mut mint_amount = "0".to_string();
                let mut deposit_amount = "0".to_string();

                let fist_tx_log: Log = serde_json::from_str(tx_responses[index].logs[0].to_string().as_str())?;
                
                for i in 0..fist_tx_log.events.len() {
                    if fist_tx_log.events[i].type_field == "wasm" {
                        for ii in 0..fist_tx_log.events[i].attributes.len() {
                            if fist_tx_log.events[i].attributes[ii].key == "mint_amount" {
                                mint_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();
                            }
                            if fist_tx_log.events[i].attributes[ii].key == "deposit_amount" { 
                                deposit_amount = fist_tx_log.events[i].attributes[ii].value.to_owned();   
                            }
                        }
                    }
                }  
                let mint_amount = rust_decimal::Decimal::from_str(mint_amount.as_str());
                let deposit_amount = rust_decimal::Decimal::from_str(deposit_amount.as_str());

                match (mint_amount,deposit_amount) {
                    (Ok(mint),Ok(deposit)) => {
                        let exchange_rate = deposit.checked_div(mint);
                        if let Some(exchange) = exchange_rate {
                             stablecoin_deposits.push( DepositStableLog { height: height-count, timestamp: tx_responses[index].timestamp.timestamp(), mint_amount: mint, deposit_amount: deposit, exchange_rate: exchange});
                        }
                    },
                    _ => {

                    }
                }  


            }
        }
        count = count +1;
    }

    if stablecoin_deposits.len()<1 && start.elapsed().as_secs() >= 60*3 {
        return Err(anyhow!("Unexpected Error: Timeout!"));
    }

    Ok(ResponseResult::StablecoinDeposits(Response{
        height: height.to_string(),
        result: stablecoin_deposits
        }))
}

pub async fn blocks_per_year_query() -> anyhow::Result<ResponseResult> {
    let latest_block = query_core_latest_block().await?;      
    let historic_block_height = latest_block.block.header.height - (60*60*24*30)/6;
    let historic_block = query_core_block_at_height(historic_block_height as u64).await?;

    let time_difference = latest_block.block.header.time.timestamp_millis() - historic_block.block.header.time.timestamp_millis();
    let height_difference = latest_block.block.header.height - historic_block.block.header.height;
    let block_per_millis = height_difference as f64 / time_difference as f64;
    let blocks_per_year = block_per_millis *1000f64 * 60f64 * 60f64 * 24f64 * 365f64;

    Ok(ResponseResult::Blocks(Response{
        height: latest_block.block.header.height.to_string(),
        result: BlocksPerYearResponse {
                blocks_per_year: blocks_per_year,
                blocks_per_millis: block_per_millis,
                latest_block: serde_json::to_string_pretty(&latest_block)?,
                historic_block: serde_json::to_string_pretty(&historic_block)?,
            }
        }))
}

// blunaHubState: state, anchorprotocol, bLunaHub
// anchor_protocol_state: state, anchorprotocol, mmMarket 

pub async fn state_query_msg(protocol: String, contract: String, gas_prices: GasPrices) -> anyhow::Result<ResponseResult> {
	let query = r#"{"state":{}}"#;   
 	let contract_addr = get_contract(&protocol,&contract);
	
	let res: String = get_fcd_or_lcd_query(&contract_addr,&query, &gas_prices).await?; 
    //println!("{:?}",&res);
    match contract.as_str() {
        "mmMarket" => {
            let response: Response<MarketStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::mmMarket(response)));
        },
        "bLunaHub" => {
            let response: Response<BLunaStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::State(StateResponse::bLunaHub(response))); 
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// aust_to_ust: epoch_state, anchorprotocol, mmMarket
pub async fn epoch_state_query_msg(protocol: String, contract: String, gas_prices: GasPrices) -> anyhow::Result<ResponseResult> {
    let query = r#"{"epoch_state":{}}"#;  
    let contract_addr = get_contract(&protocol,&contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query, &gas_prices).await?; 
    
    match contract.as_str() {
        "mmMarket" => {
            let res: Response<MarketEpochStateResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::EpochState(EpochStateResponse::mmMarket(res)));
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    } 
}

// anchor_protocol_interest_model_config: anchorprotocol, mmInterestModel
// anchor_protocol_collector_config: anchorprotocol, collector 
pub async fn config_query_msg(protocol: String, contract: String, gas_prices: GasPrices) -> anyhow::Result<ResponseResult> {
    let query = r#"{"config":{}}"#;  
    let contract_addr = get_contract(&protocol,&contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query, &gas_prices).await?; 

    match contract.as_str() {
        "mmInterestModel" => { 
            let response: Response<InterestModelConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::mmInterestModel(response)));
        },
        "collector" => {
            let response: Response<CollectorConfigResponse> = serde_json::from_str(&res)?;
            return Ok(ResponseResult::Config(ConfigResponse::Collector(response))); 
        },
        _ => {
            return Err(anyhow!("Unexpected Error: Unknown Contract {:?}",contract));
        }
    }
}

// core_swap usdr uusd
pub async fn native_token_core_swap(from_native_token: String, to_native_token: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    let res: String = query_core_market_swap_rate(&from_native_token,&to_native_token,&gas_prices).await?; 
    let res: Response<CoreSwapResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::CoreSwap(res))
}

// luna_to_bluna: uluna, anchorprotocol,terraswapblunaLunaPair
// luna_to_ust: uluna, terraswap, uusd_uluna_pair_contract
// sdt_to_uluna: usdr, terraswap, usdr_uluna_pair_contract
// ust_to_luna: uusd, terraswap, uusd_uluna_pair_contract
// ust_to_psi: uusd, nexusprotocol, Psi-UST pair
// ust_to_anc: uusd, anchorprotocol, terraswapAncUstPair
pub async fn native_token_to_swap_pair(protocol: String, native_token: String, pair_contract: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"native_token":{"denom":"my_native_token"}}}}}"#.replace("my_native_token", &native_token); 
    let contract_addr = get_contract(&protocol, &pair_contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?; 
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}

// bluna_to_luna: anchorprotocol, bLunaToken, terraswapblunaLunaPair
// nluna_to_psi: nexusprotocol, nLuna token, Psi-nLuna pair
// psi_to_nluna: nexusprotocol, Psi token, Psi-nLuna pair
// psi_to_ust: nexusprotocol,  Psi token, Psi-UST pair
// anc_to_ust: anchorprotocol, ANC, terraswapAncUstPair 
pub async fn cw20_to_swap_pair(protocol: String, token_contract: String, pair_contract: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"token":{"contract_addr":"my_cw20_contract_addr"}}}}}"#.replace("my_cw20_contract_addr", &get_contract(&protocol,&token_contract)); 
    let contract_addr = get_contract(&protocol, &pair_contract);
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}

pub async fn masset_to_ust(masset: String, gas_prices: GasPrices) -> anyhow::Result<ResponseResult> {
    //let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"native_token":{"denom":"my_native_token"}}}}}"#.replace("my_native_token", "uusd"); 
    let query = r#"{"simulation":{"offer_asset":{"amount":"1000000","info":{"token":{"contract_addr":"my_cw20_contract_addr"}}}}}"#.replace("my_cw20_contract_addr", &get_mirrorprotocol_assets(&masset,"token"));
    let contract_addr = get_mirrorprotocol_assets(&masset,"pair");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<SimulationResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Simulation(res))
}  

pub async fn masset_oracle_price(masset: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.mirror.finance/contracts/oracle#price
    let query = r#"{"price": {"base_asset": "my_cw20_contract_addr","quote_asset": "uusd"}}"#.replace("my_cw20_contract_addr", &get_mirrorprotocol_assets(&masset,"token")); 
    let contract_addr = get_contract("mirrorprotocol","oracle");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<PriceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Price(res))
}


pub async fn anchor_protocol_borrower_limit(wallet_acc_address: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/overseer#borrowlimitresponse
    let query = r#"{"borrow_limit": {"borrower": "wallet_acc_address"}}"#.replace("wallet_acc_address", &wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","mmOverseer");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<BorrowLimitResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowLimit(res))
}

pub async fn anchor_protocol_borrower_info(wallet_acc_address: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/money-market/market#borrowerinforesponse
    let query = r#"{"borrower_info": {"borrower": "wallet_acc_address"}}"#.replace("wallet_acc_address", &wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","mmMarket");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<BorrowInfoResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::BorrowInfo(res))
} 

pub async fn anchor_protocol_anc_balance(wallet_acc_address: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.terra.money/Tutorials/Smart-contracts/Manage-CW20-tokens.html#checking-cw20-balance
    let query = r#"{"balance": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", &wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","ANC");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
} 

pub async fn anchor_protocol_balance(wallet_acc_address: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.terra.money/Tutorials/Smart-contracts/Manage-CW20-tokens.html#checking-cw20-balance
    let query = r#"{"balance": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", &wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","aTerra");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<BalanceResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balance(res))
} 

pub async fn terra_balances(wallet_acc_address: String) ->  anyhow::Result<ResponseResult> { 
    let res: String = query_core_bank_balances(&wallet_acc_address).await?;
    let res: Response<Vec<Coin>> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Balances(res))
} 



pub async fn anchor_protocol_staker(wallet_acc_address: String, gas_prices: GasPrices) ->  anyhow::Result<ResponseResult> {
    // https://docs.anchorprotocol.com/smart-contracts/anchor-token/gov#staker
    let query = r#"{"staker": {"address": "wallet_acc_address"}}"#.replace("wallet_acc_address", &wallet_acc_address); 
    let contract_addr = get_contract("anchorprotocol","gov");
    
    let res: String = get_fcd_or_lcd_query(&contract_addr,&query,&gas_prices).await?;
    let res: Response<StakerResponse> = serde_json::from_str(&res)?;
    Ok(ResponseResult::Staker(res))
}  

pub async fn query_api_distribution_apy() ->  anyhow::Result<ResponseResult> {
    // {"anc_price":"3.591430997773948743","height":5549202,"timestamp":1638643455550,"anc_emission_rate":"20381363.851572310123647620","total_liabilities":"1479450867061244.823197164919607620","distribution_apy":"0.230403324402556547"}
    let res: String = query_api("https://api.anchorprotocol.com/api/v2/distribution-apy").await?;
    let res: DistributionApyResponse = serde_json::from_str(&res)?;
    Ok(ResponseResult::DistributionApy(res))
}

pub async fn query_api_gov_reward() ->  anyhow::Result<ResponseResult> {
    // {"height":5549202,"timestamp":1638643455550,"gov_share_index":"1.045394739707661316","current_apy":"0.087490822032878940"}
    let res: String = query_api("https://api.anchorprotocol.com/api/v2/gov-reward").await?;
    let res: GovRewardResponse = serde_json::from_str(&res)?;
    Ok(ResponseResult::GovReward(res))
}
 