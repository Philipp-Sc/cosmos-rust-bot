use rust_decimal::Decimal;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub pause_requested: bool,
    pub hot_reload: bool,
    pub remove: bool,
    pub test: bool,
    pub terra_wallet_address: Option<String>,
    pub anchor_protocol_auto_repay: bool,
    pub anchor_protocol_auto_borrow: bool,
    pub anchor_protocol_auto_stake: bool,
    pub anchor_protocol_auto_farm: bool,
    pub terra_market_info: bool,
    pub anchor_general_info: bool,
    pub anchor_account_info: bool,
    pub trigger_percentage: Decimal,
    pub target_percentage: Decimal,
    pub borrow_percentage: Decimal,
    pub min_ust_balance: Decimal,
    pub gas_adjustment_preference: Decimal,
    pub max_tx_fee: Decimal,
    pub ust_balance_preference: Decimal,
}
impl Default for UserSettings {
    fn default() -> UserSettings {
        UserSettings {
            pause_requested: false,
            hot_reload: false,
            remove: false,
            test: true,
            terra_wallet_address: None,
            anchor_protocol_auto_repay: false,
            anchor_protocol_auto_borrow: false,
            anchor_protocol_auto_stake: false,
            anchor_protocol_auto_farm: false,
            terra_market_info: true,
            anchor_general_info: true,
            anchor_account_info: false,
            trigger_percentage: Decimal::from_str("0.9").unwrap(),
            target_percentage: Decimal::from_str("0.72").unwrap(),
            borrow_percentage: Decimal::from_str("0.5").unwrap(),
            max_tx_fee: Decimal::from_str("5").unwrap(),
            gas_adjustment_preference: Decimal::from_str("1.2").unwrap(),
            min_ust_balance: Decimal::from_str("10").unwrap(),
            ust_balance_preference: Decimal::from_str("20").unwrap(),
        }
    }
}


pub fn requirement_list() ->  Vec<(&'static str, i32, Vec<&'static str>)>  {

       // note: around every 6s a new block is generated. 
       let fast: i32 = 10;      // 10s for short lived information
       let medium: i32 = 60;    // 1m  for short lived information
       let slow: i32 = 60*10;   // 10m for relative constant information. 


       // (key, target_refresh_time, dependency_tag)
       vec![
        /* <from settings> */ 
        ("trigger_percentage", fast, vec!["anchor_account","anchor_auto_repay"]),
        ("target_percentage", fast, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        ("borrow_percentage", fast, vec!["anchor_auto_borrow"]),
        ("gas_adjustment_preference",fast, vec!["anchor_account","anchor_auto_lp","anchor_auto_stake","anchor_auto_stake_airdrops","anchor_auto_repay","anchor_auto_borrow"]),
        ("min_ust_balance", fast, vec!["anchor_account","anchor_auto_lp","anchor_auto_stake","anchor_auto_stake_airdrops","anchor_auto_repay","anchor_auto_borrow"]),
        ("ust_balance_preference", fast, vec!["anchor_auto_repay"]),
        ("max_tx_fee", fast, vec!["anchor_auto_lp","anchor_auto_stake","anchor_auto_stake_airdrops","anchor_auto_repay","anchor_auto_borrow"]),
        /* <for gas fees>*/
        ("gas_fees_uusd", medium, vec!["market","anchor","anchor_account","anchor_auto_lp","anchor_auto_stake","anchor_auto_stake_airdrops","anchor_auto_repay","anchor_auto_borrow"]),
        ("tax_rate", medium, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        ("tax_caps", medium, vec!["anchor_auto_repay","anchor_auto_borrow"]),
        /**/
        ("terra_balances", fast, vec!["anchor_auto_lp","anchor_auto_stake","anchor_auto_stake_airdrops","anchor_auto_repay","anchor_auto_borrow"]),
        /* <market_info> */
        /* core_tokens */
        ("core_swap uusd usdr", fast, vec!["market"]),
        ("core_swap usdr uluna", fast, vec!["market"]),
        ("core_swap uluna uusd", fast, vec!["market"]),
        // "simulation terraswap usdr usdr_uluna_pair_contract",
        // "simulation terraswap uluna uusd_uluna_pair_contract",
        /* anchor_tokens */
        ("simulation anchorprotocol uluna terraswapblunaLunaPair",fast, vec!["market","anchor_account"]),
        ("state anchorprotocol bLunaHub", fast, vec!["market","anchor_account"]),
        ("simulation_cw20 anchorprotocol ANC terraswapAncUstPair", fast, vec!["market","anchor_account","anchor_auto_lp","anchor_auto_stake"]),
        ("epoch_state anchorprotocol mmMarket", fast, vec!["anchor","market","anchor_account","anchor_auto_repay"]),
        /* nexus_tokens */
        ("simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair", fast, vec!["market"]),
        ("simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair", fast, vec!["market"]),
        /* mirror_tokens */
        ("simulation_cw20 uusd mir", fast, vec!["market"]),
        ("simulation_cw20 uusd m_tsla", fast, vec!["market"]),
        ("simulation_cw20 uusd m_btc", fast, vec!["market"]),
        ("simulation_cw20 uusd m_eth", fast, vec!["market"]),
        ("simulation_cw20 uusd m_spy", fast, vec!["market"]),
        /* <other> */
        /* <anchor_protocol> */
        ("state anchorprotocol mmMarket", fast, vec!["anchor","anchor_account"]),
        ("api/v2/distribution-apy", fast, vec!["anchor","anchor_account","anchor_auto_lp","anchor_auto_stake"]),
        ("api/v2/gov-reward", fast, vec!["anchor","anchor_account","anchor_auto_stake"]),
        ("config anchorprotocol mmInterestModel", fast, vec!["anchor","anchor_account"]),
        //("config anchorprotocol collector",every_minute),
        /* <anchor_protocol account> */ 
        ("anchor_airdrops", fast, vec!["anchor_auto_stake_airdrops"]),
        ("borrow_limit", fast, vec!["anchor_account","anchor_auto_repay","anchor_auto_borrow"]),
        ("borrow_info", fast, vec!["anchor_account","anchor_auto_lp","anchor_auto_stake","anchor_auto_repay","anchor_auto_borrow"]),
        ("balance", fast, vec!["anchor_account","anchor_auto_repay","anchor_auto_borrow"]),
        ("anc_balance", fast, vec!["anchor_account","anchor_auto_stake"]),
        ("staker", fast, vec!["anchor_account","anchor_auto_stake"]),
        ("blocks_per_year", slow, vec!["market","anchor","anchor_account"]), 
        ("earn_apy", slow, vec!["anchor","anchor_account"]),
        ("anchor_protocol_whitelist", slow, vec!["anchor_account"]),
        /* <meta data> */ 
        ("anchor_protocol_txs_claim_rewards", slow, vec!["anchor","anchor_account","anchor_auto_lp","anchor_auto_stake"]), 
        ("anchor_protocol_txs_staking", slow, vec!["anchor","anchor_account","anchor_auto_stake"]), 
        ("anchor_protocol_txs_redeem_stable", slow, vec!["anchor_auto_repay"]), 
        ("anchor_protocol_txs_deposit_stable", slow, vec!["anchor_auto_borrow"]), 
        ("anchor_protocol_txs_borrow_stable", slow, vec!["anchor_auto_borrow"]), 
        ("anchor_protocol_txs_repay_stable", slow, vec!["anchor_auto_repay"]), 
//        ("anchor_protocol_txs_provide_liquidity", slow, vec!["anchor_auto_lp"]), 
//        ("anchor_protocol_txs_staking_lp", slow, vec!["anchor_auto_lp"]), 
        ("txs_provide_to_spec_anc_ust_vault", slow, vec!["anchor_auto_lp"]), 
//        ("api/v2/ust-lp-reward", slow, vec!["anchor_auto_lp"]), 
        ("api/data?type=lpVault", slow, vec!["anchor_auto_lp"]),  
        ]

 }

 pub fn my_requirement_keys(user_settings: &UserSettings) -> Vec<&str> {

       let args = settings_to_key_list(user_settings);
       let req = requirement_list();
       let mut req_keys: Vec<&str> = Vec::new();  
        for i in 0..req.len() {
            for x in &args {
                if req[i].2.contains(x) {
                    req_keys.push(req[i].0); 
                    break;
                }
            }
        }
        req_keys
 }

pub fn my_bot_keys(user_settings: &UserSettings) -> Vec<&str> {

    let mut args: Vec<&str> = Vec::new();
    if user_settings.anchor_protocol_auto_stake {
        args.push("anchor_auto_stake");
    }
    if user_settings.anchor_protocol_auto_farm {
        args.push("anchor_auto_farm");
    }
    if user_settings.anchor_protocol_auto_repay {
        args.push("anchor_auto_repay");
    }
    if user_settings.anchor_protocol_auto_borrow {
        args.push("anchor_auto_borrow");
    }
    args
}


 pub fn my_requirement_list(user_settings: &UserSettings) -> Vec<(&'static str, i32, Vec<&'static str>)> {

    let args = settings_to_key_list(user_settings);
    let req = requirement_list();
    let mut req_new = Vec::new();
    for i in 0..req.len() {
          for x in &args {
                if req[i].2.contains(x) {
                        req_new.push((req[i].0,req[i].1,req[i].2.clone()));
                        break;
            }
        }
    }
    req_new
 }

fn settings_to_key_list(user_settings: &UserSettings) -> Vec<&str> {
    let mut args: Vec<&str> = Vec::new();
    if user_settings.anchor_protocol_auto_stake {
        args.push("anchor_auto_stake");
    }
    if user_settings.anchor_protocol_auto_farm {
        args.push("anchor_auto_lp");
    }
    if user_settings.anchor_protocol_auto_repay {
        args.push("anchor_auto_repay");
    }
    if user_settings.anchor_protocol_auto_borrow {
        args.push("anchor_auto_borrow");
    }
    if user_settings.terra_market_info {
        args.push("market");
    }
    if user_settings.anchor_general_info {
        args.push("anchor");
    }
    if user_settings.anchor_account_info {
        args.push("anchor_account");
    }
    args
}