pub mod lazy_logs;

 
use lazy_logs::display::*; 
use lazy_logs::control::view::interface::model::{MaybeOrPromise};
  
use lazy_logs::control::view::*;
use lazy_logs::control::view::interface::*; 

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*; 


pub async fn display_anchor_account(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize,is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {

    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();

    let mut anchor_view: Vec<(String,usize)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol Account**\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

    // AIRDROP TEST
/*
    anchor_view.push((format!("{}{}","\n   [Airdrops]".truecolor(75,219,75),"  luna staking airdrops:   ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anchor_airdrops_to_string(tasks.clone())));
    anchor_tasks.push(t);
    *offset += 1;

    
    println!("{}",anchor_claim_and_stake_airdrops(tasks.clone(),"--").await);

*/

    //anchor_view.push((format!("{}{}","\n   [Liquidation Queue]".truecolor(75,219,75),"    withdrawals:             ".purple().to_string()),*offset));
    //*offset += 1;

    anchor_view.push((format!("{}{}","\n\n   [Borrow]".truecolor(75,219,75),"    loan amount:             ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_loan_amount_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST (borrow limit: ".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_limit_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    loan to value:           ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ltv_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    pending rewards:         ".purple().to_string()),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" UST (=".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((" ANC)".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anchor_claim_and_stake_transaction_gas_fees_ratio_to_string(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push(("  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim_and_stake(tasks.clone(),3)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST)".purple().to_string(),*offset));
    *offset += 1;
   
    anchor_view.push((format!("{}{}","\n\n   [Earn]".truecolor(75,219,75),"      deposit:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_ust_deposited_to_string(tasks.clone(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_balance_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" aUST)".purple().to_string(),*offset));
    *offset += 1;
 

    //anchor_view.push((format!("{}{}","\n   [Borrow]".truecolor(75,219,75),"    fee to claim & stake:    ".to_string(),*offset));
   /*
    let available_liquidity_from_ust_deposit = borrower_deposit_liquidity_to_string(tasks.clone(),2).await;
    display_add(format!("   [Earn]    deposit liquidity:    {}",available_liquidity_from_ust_deposit), 23 as usize,2 as usize); 
   */  
    anchor_view.push((format!("{}{}","\n\n   [Gov]".truecolor(75,219,75),"       balance:                 ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_anc_deposited_to_string(tasks.clone(),false,4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC".purple().to_string(),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       staked:                  ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_in_ust_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST  (=".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(anc_staked_balance_to_string(tasks.clone(),4)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" ANC)\n\n".purple().to_string(),*offset));
    *offset += 1;
  
    //   add -> (=absolute returns) UST or ANC FOR DISTRIBUTION APR AND AUTO STAKING
  
    anchor_view.push((format!("    {}",display_add("   _    _    Net APY    Borrow APY    Distribution APY    Earn APY    Auto Staking APY (not included in Net APY)".purple().to_string(), 23 as usize,2 as usize)),*offset));
    *offset += 1;
   
    anchor_view.push((display_add("   [Anchor]    loan_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    loan_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"loan_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    target_ltv:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    target_ltv:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"target_ltv","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","total_returns",2));
    let f = Box::pin(add_format_to_result(" (=".to_string()," UST)".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","date_next",2));
    let f = Box::pin(add_format_to_result(" Next Auto Stake: ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"target_ltv","duration_next",2));
    let f = Box::pin(add_format_to_result(" (every ".to_string(),")".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("   [Anchor]    deposit_amount:    --".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","net_apr",2));
    let f = Box::pin(add_format_to_result("   [Anchor]    deposit_amount:    ".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","borrow_apr",2));
    let f = Box::pin(add_format_to_result("    -".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;


    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","distribution_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,0 as usize),*offset));
    let f = Box::pin(apy_on_collateral_by(tasks.clone(),"deposit_amount","earn_apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,0 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((display_add("--".purple().to_string(), 23 as usize,1 as usize),*offset));
    let f = Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"loan_amount","apr",2));
    let f = Box::pin(add_format_to_result("    +".to_string(),"".to_string(),f));
    let f = Box::pin(add_table_formatting(f, 23 as usize,1 as usize));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset,f);
    anchor_tasks.push(t);
    *offset += 1;

       
    // ADD ANC scenario
    // ANC -50%, -25%, 0%, + 25%, +50%, + 100%

    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }

    return anchor_tasks;
  
}
 

pub async fn display_anchor_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {
 

    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();
    let mut anchor_view: Vec<(String,usize)> = Vec::new();

    anchor_view.push(("\n  **Anchor Protocol**\n\n".truecolor(75,219,75).to_string(),*offset));
    *offset += 1;
 

    /* Expert Parameters
    let total_liabilities = total_liabilities_to_string(tasks.clone(),"state anchorprotocol mmMarket",0).await;
    display[*offset] = format!("   [Anchor] stablecoins lent:        {} UST\n",total_liabilities);
    *offset += 1;
    println!("{}",display.join(""));
    
    let stablecoins_deposited = a_terra_supply_to_string(tasks.clone(), "epoch_state anchorprotocol mmMarket",0).await; 
    display[*offset] = format!("   [Anchor] stablecoins deposited:   {} UST\n", stablecoins_deposited);
    *offset += 1;
    println!("{}",display.join(""));

    let utilization_ratio = utilization_ratio_to_string(tasks.clone(),"state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",2).await;
    display[*offset] = format!("   [Anchor] utilization ratio:       {}\n\n",utilization_ratio);
    //\n  *The utilization ratio quantifies a stablecoin's borrow demand relative to the amount of deposited stablecoins.\n
    *offset += 1;
    println!("{}",display.join(""));
    */

    /*  Expert Parameters
    let base_rate = base_rate_to_string(tasks.clone(),"config anchorprotocol mmInterestModel",10).await;
    display[*offset] = format!("   [Anchor] base rate:               {}\n",base_rate);
    *offset += 1;
    println!("{}",display.join(""));
    let interest_multiplier = interest_multiplier_to_string(tasks.clone(),10).await;
    display[*offset] = format!("   [Anchor] interest multiplier:     {}\n",interest_multiplier);
    *offset += 1;
    println!("{}",display.join(""));

    let borrow_rate = borrow_rate_to_string(tasks.clone(),"config anchorprotocol mmInterestModel","state anchorprotocol mmMarket","epoch_state anchorprotocol mmMarket",10).await;
    display[*offset] = format!("   [Anchor] borrow rate:             {}\n",borrow_rate);
    *offset += 1;
    println!("{}",display.join(""));
    */

    anchor_view.push((format!("{}{}","   [Borrow]".truecolor(75,219,75),"    net apr:                 ".purple().to_string()),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(net_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // The borrow rate equation incentivizes markets to have sufficient liquidity at their equilibrium. An increase in borrow demand is met with higher borrow rates, incentivizing repayments, and restoring market liquidity.
    anchor_view.push((" (borrow apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrow_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    // Borrower incentives: 400M (40%) tokens are linearly released to be used as borrower incentives over a period of 4 years. 
    anchor_view.push((", distribution apr: ".purple().to_string(),*offset));
    *offset += 1;


    anchor_view.push(("--".purple().to_string(),*offset));
    // TODO: figure out the distribution apy calculation from the smart contracts.
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(distribution_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}",")\n","   [Borrow]".truecolor(75,219,75),"    fee to claim:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_claim_rewards","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n\n".purple().to_string(),"   [Gov]".truecolor(75,219,75), "       ANC staking apy:         ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    // Anchor periodically distributes portion of ANC tokens purchased from protocol fees are distributed to ANC stakers to incentivize governance participation and decrease circulating ANC supply
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(staking_apy_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Gov]".truecolor(75,219,75),"       fee to stake:            ".purple().to_string()),*offset));
    *offset += 1;
 
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee(tasks.clone(),"anchor_protocol_txs_staking","fee_amount_adjusted".to_owned(),false,2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((format!("{}{}{}"," UST\n".purple().to_string(),"\n   [Earn]".truecolor(75,219,75),"      deposit apy:             ".purple().to_string()),*offset));
    *offset += 1;
    
    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(earn_apr_to_string(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push(("\n".to_string(),*offset));
    *offset += 1;
 
    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await;
    }

    return anchor_tasks;

}
 

pub async fn display_market_info(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>, new_display: &Arc<RwLock<Vec<String>>> ,offset: &mut usize, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {

    let mut market_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = vec![];

    let mut market_view: Vec<(String,usize)> = Vec::new();

    market_view.push(("\n\n  **Terra**\n\n".truecolor(84, 147, 247).to_string(),*offset)); 
    *offset += 1;


    market_view.push((format!("{}{}","   [Terra]".truecolor(84, 147, 247),"    est. blocks per year:   ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(blocks_per_year_to_string(tasks.clone(),0)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Terra]".truecolor(84, 147, 247),"    gas price:              ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(gas_price_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Terra]".truecolor(84, 147, 247),"    SDT    -> Luna:         ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(),"core_swap usdr uluna",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((" (=$".purple().to_string(),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uusd usdr",2)));
    market_tasks.push(t);
    *offset += 1;
    market_view.push((")".purple().to_string(),*offset));
    *offset += 1;
 
    market_view.push((format!("{}{}","\n   [Terra]".truecolor(84, 147, 247),"    Luna   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(core_swap_amount_to_string(tasks.clone(), "core_swap uluna uusd",2)));
    market_tasks.push(t);
    *offset += 1;
 
    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation anchorprotocol uluna terraswapblunaLunaPair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Bond]".truecolor(75,219,75),"     Luna   -> bLuna:        ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(b_luna_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"   ANC    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"   aUST   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(a_terra_exchange_rate_to_string(tasks.clone(),4)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n\n   [Nexus]".truecolor(244, 182, 199),"    nLuna  -> PSI:          ".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",2)));
    market_tasks.push(t);
    *offset += 1;
    
    market_view.push((format!("{}{}","\n   [Nexus]".truecolor(244, 182, 199),"    PSI    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Mirror]".truecolor(228, 228, 231),"   MIR    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd mir",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mTSLA  -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_tsla",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mSPY   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_spy",2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mBTC   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_btc",2)));
    market_tasks.push(t);
    *offset += 1;
     
    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mETH   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_eth",2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push(("\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, market_view).await;
    }

    return market_tasks;

}
