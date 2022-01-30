/*
 * API calls that do not query the LCD or FCD. 
 *
 */

pub mod blockchain;

use blockchain::smart_contracts::objects::*; 
use blockchain::smart_contracts::objects::meta::api::query_api;

use blockchain::smart_contracts::{airdrop_is_claimed};

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

pub async fn query_anchor_airdrops(wallet_acc_address: &str) ->  anyhow::Result<ResponseResult> {
    let res: String = query_api(&format!("https://airdrop.anchorprotocol.com/api/get?address={}&chainId=columbus-4",wallet_acc_address)).await?;
    let mut res: AnchorAirdrops = serde_json::from_str(&res)?;
    for i in 0..res.len() {
        res[i].claimable = match airdrop_is_claimed(wallet_acc_address,res[i].stage).await {
            Ok(response_result) => {response_result.as_is_claimed_response().unwrap().result.is_claimed == false},
            Err(_) => {false},
        } ;
    }
    Ok(ResponseResult::AirdropResponse(res))
}
