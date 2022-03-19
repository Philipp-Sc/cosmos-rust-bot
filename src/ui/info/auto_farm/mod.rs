use crate::ui::display::*;
use crate::state::control::model::{MaybeOrPromise,await_function};

use crate::view::*;
use crate::view::interface::*;

//use crate::action::*;

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*;
   
/**
 * Anchor Auto Farm requires that the account balance has sufficient funds.
 * Info: It will not replenish the account balance. 
 * */
pub async fn lazy_anchor_account_auto_farm_rewards(tasks: &Arc<RwLock<HashMap<String, MaybeOrPromise>>>,  new_display: &Arc<RwLock<Vec<String>>>,offset: &mut usize, is_test: bool, is_first_run: bool) -> Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> {


    let mut anchor_view: Vec<(String,usize)> = Vec::new();
    let mut anchor_tasks: Vec<(usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>)> = Vec::new();


    anchor_view.push(("\n  **Anchor Protocol Auto Farm**\n\n".truecolor(75,219,75).to_string(),*offset)); 
    *offset += 1;

     // Pending ANC Rewards
    
    anchor_view.push((format!("{}{}","\n   [Auto Borrow Rewards]".truecolor(75,219,75),"   amount:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(borrower_rewards_in_ust_to_string(tasks.clone(),  2)));
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

    // Fee to claim ANC rewards

    anchor_view.push((format!("{}{}","\n\n   [Anchor Claim]".truecolor(75,219,75),"          est. fee:          ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_tx_fee_claim(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    // Fee to provide ANC rewards to spec

    anchor_view.push((format!("{}{}","\n   [SPEC Provide]".truecolor(75,219,75),"          est. fee:          ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_spec_tx_fee_provide(tasks.clone(),2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"                anc_to_keep:       ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_keep",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"                anc_to_swap:       ".purple()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(calculate_farm_plan(tasks.clone(),"anc_to_swap",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 


    anchor_view.push((format!("{}{}","\n\n   [Anchor]".truecolor(75,219,75),"                belief_price:      ".purple()),*offset));
    *offset += 1; 

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_exchange_rate_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,2)));
    anchor_tasks.push(t);
    *offset += 1; 

    anchor_view.push((format!("{}{}","\n   [Anchor]".truecolor(75,219,75),"                max_spread:        0.001".purple()),*offset));
    *offset += 1;
 
    anchor_view.push((format!("{}{}","\n\n   [Auto Farm Rewards]".truecolor(75,219,75),"     target:            ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","value_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;

    anchor_view.push((format!("{}{}","\n   [Auto Farm]".truecolor(75,219,75),"             next:              ".purple().to_string()),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(estimate_anchor_protocol_next_claim_and_stake_tx(tasks.clone(),"farming","loan_amount","date_next",2)));
    anchor_tasks.push(t);
    *offset += 1;

    anchor_view.push((" UST".purple().to_string(),*offset));
    *offset += 1;
     
    let mut field = "result:  ";

    if is_test {
        field = "estimate:";
    }

    anchor_view.push((format!("{}{}","\n\n   [Auto Farm]".truecolor(75,219,75),format!("             {}          ",field.purple())),*offset));
    *offset += 1;

    anchor_view.push(("--".purple().to_string(),*offset));

    // display task here
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(await_function(tasks.clone(),"anchor_auto_farm".to_owned())));
    anchor_tasks.push(t);
    *offset += 1;
    
    anchor_view.push(("\n\n".to_string(),*offset));
    *offset += 1;


    if is_first_run {
        add_view_to_display(&new_display, anchor_view).await; 
    }     

    return anchor_tasks;
}