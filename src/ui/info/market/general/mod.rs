
use crate::ui::display::*;

use crate::state::control::model::{MaybeOrPromise};
  
use crate::view::interface::*; 

use std::collections::HashMap;
use core::pin::Pin;
use core::future::Future;

use std::sync::Arc; 
use tokio::sync::RwLock;   
use colored::*; 

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
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation anchorprotocol uluna terraswapblunaLunaPair",false,4)));
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
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 anchorprotocol ANC terraswapAncUstPair",false,2)));
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
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol nLunaToken Psi-nLuna_Pair",false,2)));
    market_tasks.push(t);
    *offset += 1;
    
    market_view.push((format!("{}{}","\n   [Nexus]".truecolor(244, 182, 199),"    PSI    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(), "simulation_cw20 nexusprotocol PsiToken Psi-UST_Pair",false,4)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n\n   [Mirror]".truecolor(228, 228, 231),"   MIR    -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd mir",false,2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mTSLA  -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_tsla",false,2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mSPY   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_spy",false,2)));
    market_tasks.push(t);
    *offset += 1;

    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mBTC   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_btc",false,2)));
    market_tasks.push(t);
    *offset += 1;
     
    market_view.push((format!("{}{}","\n   [Mirror]".truecolor(228, 228, 231),"   mETH   -> UST:          $".purple()),*offset));
    *offset += 1;

    market_view.push(("--".purple().to_string(),*offset));
    let t: (usize,Pin<Box<dyn Future<Output = String> + Send + 'static>>) = (*offset, Box::pin(simulation_swap_return_amount_to_string(tasks.clone(),"simulation_cw20 uusd m_eth",false,2)));
    market_tasks.push(t);
    *offset += 1; 

    market_view.push(("\n".to_string(),*offset));
    *offset += 1;

    if is_first_run {
        add_view_to_display(&new_display, market_view).await;
    }

    return market_tasks;

}
