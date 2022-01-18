/*
 * API calls that do not query the LCD or FCD. 
 *
 */

pub mod blockchain;

use blockchain::smart_contracts::objects::*; 
use blockchain::smart_contracts::objects::meta::api::query_api;

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
