/*
 * API calls that do not query the LCD or FCD. 
 *
 */

pub mod blockchain;

use blockchain::smart_contracts::objects::*; 
use blockchain::smart_contracts::objects::meta::api::{query_api,/*query_api_with_post*/};

use blockchain::smart_contracts::{airdrop_is_claimed};

use blockchain::smart_contracts::objects::meta::api::data::endpoints::{get_spectrumprotocol_api,get_anchorprotocol_airdrop_api,get_anchorprotocol_api};


use anyhow::anyhow; 

pub async fn query_api_distribution_apy() ->  anyhow::Result<ResponseResult> {
    // {"anc_price":"3.591430997773948743","height":5549202,"timestamp":1638643455550,"anc_emission_rate":"20381363.851572310123647620","total_liabilities":"1479450867061244.823197164919607620","distribution_apy":"0.230403324402556547"}
    let res: String = query_api(format!("{}/api/v2/distribution-apy",get_anchorprotocol_api()).as_str()).await?;
    let res: DistributionApyResponse = serde_json::from_str(&res)?;
    Ok(ResponseResult::DistributionApy(res))
}

pub async fn query_api_gov_reward() ->  anyhow::Result<ResponseResult> {
    // {"height":5549202,"timestamp":1638643455550,"gov_share_index":"1.045394739707661316","current_apy":"0.087490822032878940"}
    let res: String = query_api(format!("{}/api/v2/gov-reward",get_anchorprotocol_api()).as_str()).await?;
    let res: GovRewardResponse = serde_json::from_str(&res)?;
    Ok(ResponseResult::GovReward(res))
}

pub async fn query_api_anc_ust_lp_reward() ->  anyhow::Result<ResponseResult> {
    // {"anc_price":"1.899145683334791626","height":6435861,"timestamp":1644559466155,"apy":"0.619588707342893344","total_pool":"146714700691850.000000000000000000"}
    let res: String = query_api(format!("{}/api/v2/ust-lp-reward",get_anchorprotocol_api()).as_str()).await?;
    let res: LPRewardResponse = serde_json::from_str(&res)?;
    Ok(ResponseResult::LPReward(res))
}

pub async fn query_api_spec_anc_ust_lp_reward() ->  anyhow::Result<ResponseResult> {
    //let res: String = query_api_with_post("https://api.astroport.fi/graphql",r##"{"query":"\n  query {\n    pools {\n      pool_address\n      token_symbol\n      trading_fee\n      pool_liquidity\n      _24hr_volume\n      trading_fees {\n        apy\n        apr\n        day\n      }\n      astro_rewards {\n        apy\n        apr\n        day\n      }\n      protocol_rewards {\n        apy\n        apr\n        day\n      }\n      total_rewards {\n        apy\n        apr\n        day\n      }\n    }\n  }\n"}"##).await?;
    let res: String = query_api(format!("{}/api/data?type=lpVault",get_spectrumprotocol_api()).as_str()).await?;
   
    let res: serde_json::Value = serde_json::from_str(&res)?;
    let pairs = res.get("stat").ok_or(anyhow!("no stat"))?.get("pairs").ok_or(anyhow!("no pairs"))?;
    if pairs.get("Astroport|terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76|uusd") != None {
            let data: SpecAstroVaultResponse= serde_json::from_str(pairs.get("Astroport|terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76|uusd").unwrap().to_string().as_str())?;
            return Ok(ResponseResult::SpecAstroVault(data));
    }
    Err(anyhow!("no result"))
}





pub async fn query_anchor_airdrops(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    let res: String = query_api(&format!("{}/api/get?address={}&chainId=columbus-4",get_anchorprotocol_airdrop_api(),wallet_acc_address)).await?;
    let mut res: AnchorAirdrops = serde_json::from_str(&res)?;
    for i in 0..res.len() {
        res[i].claimable = match airdrop_is_claimed(wallet_acc_address,res[i].stage).await {
            Ok(response_result) => {response_result.as_is_claimed_response().unwrap().result.is_claimed == false},
            Err(_) => {false},
        };
    } 
    Ok(ResponseResult::AirdropResponse(res))
}
