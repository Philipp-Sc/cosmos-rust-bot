/*
 * Terra Contract Addresses
 *
 */

use serde_json::json;

pub fn get_contract(source: &str, contract: &str) -> String {
	let contracts = json!({
							"mirrorprotocol": {
								// https://docs.mirror.finance/networks
								  "airdrop": "terra1kalp2knjm4cs3f59ukr4hdhuuncp648eqrgshw",
								  "collector": "terra1s4fllut0e6vw0k3fxsg4fs6fm2ad6hn0prqp3s",
								  "community": "terra1x35fvy3sy47drd3qs288sm47fjzjnksuwpyl9k",
								  "factory": "terra1mzj9nsxx0lxlaxnekleqdy8xnyw2qrh3uz6h8p",
								  "gov": "terra1wh39swv7nq36pnefnupttm2nr96kz7jjddyt2x",
								  "mint": "terra1wfz7h3aqf4cjmjcvc6s8lxdhh7k30nkczyf0mj",
								  "oracle": "terra1t6xe0txzywdg85n6k8c960cuwgh6l8esw6lau9",
								  "collateralOracle": "terra1pmlh0j5gpzh2wsmyd3cuk39cgh2gfwk6h5wy9j",
								  "staking": "terra17f7zu97865jmknk7p2glqvxzhduk78772ezac5",
								  "lock": "terra169urmlm8wcltyjsrn7gedheh7dker69ujmerv2",
								  "mirrorToken": "terra15gwkyepfc6xgca5t5zefzwy42uts8l2m4g40k6",
								  "terraswapFactory": "terra1ulgw0td86nvs4wtpsc80thv6xelk76ut7a7apj",
								  "shortReward": "terra16mlzdwqq5qs6a23250lq0fftke8v80sapc5kye",
							},
							"anchorprotocol": {  
								"SPEC ANC-UST VAULT": "terra10u9342cdwwqpe4wz9mf2c00ytlcr847wpe0xh4",
								"ANC-UST LP": "terra1wmaty65yt7mjw6fjfymkd9zsm6atsq82d9arcd",
								"ANC-UST LP Minter": "terra1qr2k6yjjd5p2kaewqvg93ag74k6gyjr7re37fs",
							// https://docs.anchorprotocol.com/developers-terra/anchor-cli
							// https://docs.anchorprotocol.com/smart-contracts/deployed-contracts
							    "bLunaHub": "terra1mtwph2juhj0rvjz7dy92gvl6xvukaxu8rfv8ts",
							    "bLunaToken": "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp",
							    "bLunaReward": "terra17yap3mhph35pcwvhza38c2lkj7gzywzy05h7l0",
							    "bLunaAirdrop": "terra199t7hg7w5vymehhg834r6799pju2q3a0ya7ae9",
							    "mmInterestModel": "terra1kq8zzq5hufas9t0kjsjc62t2kucfnx8txf547n",
							    "mmOracle": "terra1cgg6yef7qcdm070qftghfulaxmllgmvk77nc7t",
							    "mmMarket": "terra1sepfj7s0aeg5967uxnfk4thzlerrsktkpelm5s",
							    "mmOverseer": "terra1tmnqgvg567ypvsvk6rwsga3srp7e3lg6u0elp8",
							    "mmCustody": "terra1ptjp2vfjrwh0j0faj9r6katm640kgjxnwwq9kn",
							    "mmLiquidation": "terra1w9ky73v4g7v98zzdqpqgf3kjmusnx4d4mvnac6",
							    "mmDistributionModel": "terra14mufqpr5mevdfn92p4jchpkxp7xr46uyknqjwq",
							    "aTerra": "terra1hzh9vpxhsk8253se0vv5jj6etdvxu3nv8z07zu",
							    "terraswapblunaLunaPair": "terra1jxazgm67et0ce260kvrpfv50acuushpjsz2y0p",
							    "terraswapblunaLunaLPToken": "terra1nuy34nwnsh53ygpc4xprlj263cztw7vc99leh2",
							    "terraswapAncUstPair": "terra1gm5p3ner9x9xpwugn9sp6gvhd0lwrtkyrecdn3",
							    "terraswapAncUstLPToken": "terra1gecs98vcuktyfkrve9czrpgtg0m3aq586x6gzm",
							    "gov": "terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5",
							    "distributor": "terra1mxf7d5updqxfgvchd7lv6575ehhm8qfdttuqzz",
							    "collector": "terra14ku9pgw5ld90dexlyju02u4rn6frheexr5f96h",
							    "community": "terra12wk8dey0kffwp27l5ucfumczlsc9aned8rqueg",
							    "staking": "terra1897an2xux840p9lrh6py3ryankc6mspw49xse3",
							    "ANC": "terra14z56l0fp2lsf86zy3hty2z47ezkhnthtr9yq76",
							    "airdrop": "terra146ahqn6d3qgdvmj8cj96hh03dzmeedhsf0kxqm",
							    "investor_vesting": "terra1pm54pmw3ej0vfwn3gtn6cdmaqxt0x37e9jt0za",
							    "team_vesting": "terra10evq9zxk2m86n3n3xnpw28jpqwp628c6dzuq42",
							    "Anchor Liquidation Queue":"terra1e25zllgag7j9xsun3me4stnye2pcg66234je3u",
							    "Anchor bETH Token":"terra1dzhzukyezv0etz22ud940z7adyv7xgcjkahuun"
							  },
							"terraswap": { 
							// https://docs.terraswap.io/docs/contract_resources/contract_addresses/
							// see for more contracts https://assets.terra.money/cw20/pairs.json
							// see for more contracts https://github.com/terra-money/assets/
							// even more https://docs.terraswap.io/docs/contract_resources/contract_addresses/
								"uusd_uluna_pair_contract":"terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6",
								"usdr_uluna_pair_contract":"terra1vs2vuks65rq7xj78mwtvn7vvnm2gn7adjlr002"
							},
							"nexusprotocol": { 
							    // https://docs.nexusprotocol.app/launch/smart-contracts/deployed-contracts
								// https://github.com/Nexus-Protocol/documentation/blob/master/SC_api_for_frontend.md
								"NexusGovernance":"terra1xrk6v2tfjrhjz2dsfecj40ps7ayanjx970gy0j",
								"MainCommunityPool":"terra16mtntdg7rkvt3q5hcz8nqr6hj8mme5jmrtk8y2",
								"SecondaryCommunityPool":"terra1cuydpfxaen3m0l00nkn29s8kks0gk9pjvq7ep7",
								"PsiToken":"terra12897djskt9rge8dtmm86w654g7kzckkd698608",
								"nLunaToken":"terra10f2mt82kjnkxqj2gepgwl637u2w4ue2z5nhz5j",
								"nLuna_token_config_holder":"terra1rsupnpjmnjj93a8s7twgmutvknuvzvktcd32n0",
								"nETH token":"terra178v546c407pdnx5rer3hu8s2c0fc924k74ymnn",
								"nETH_token_config_holder":"terra1v58xln8ry8lx4pdt2lzpw57grksewtgqehuvah",
								"VestingContract(Investment&Team)":"terra15jggavtx37dsmfjy6sed7nnyccw2v5298xlt5g",
								"Psi_Airdrop_contract_for_ANC_Gov_stakers":"terra1992lljnteewpz0g398geufylawcmmvgh8l8v96",
								"basset_vault_strategy_for_bluna":"terra1fxc9epfhest8sndrayfkxc9y6arva4sqtp7drg",
								"basset_vault_for_bluna":"terra1cda4adzngjzcn8quvfu2229s8tedl5t306352x",
								"basset_vault_strategy_for_beth":"terra1h3y2mk0dwfy72fkwcm8h62y6uyy4k27m2m68kt",
								"basset_vault_for_beth":"terra1tfrecwlvzcv9h697q3g0d08vd53ssu5w4war4n",
								"Psi-UST_Pair":"terra163pkeeuwxzr0yhndf8xd2jprm9hrtk59xf7nqf",
								"Psi-UST_LP_token":"terra1q6r8hfdl203htfvpsmyh8x689lp2g0m7856fwd",
								"Psi-UST_staking":"terra12kzewegufqprmzl20nhsuwjjq6xu8t8ppzt30a",
								"Psi-nLuna_Pair":"terra1zvn8z6y8u2ndwvsjhtpsjsghk6pa6ugwzxp6vx",
								"Psi-nLuna_LP_token":"terra1tuw46dwfvahpcwf3ulempzsn9a0vhazut87zec",
								"Psi-nLuna_staking":"terra1hs4ev0ghwn4wr888jwm56eztfpau6rjcd8mczc",
								"Psi-nETH_pair":"terra14zhkur7l7ut7tx6kvj28fp5q982lrqns59mnp3",
								"Psi-nETH_LP_token":"terra1y8kxhfg22px5er32ctsgjvayaj8q36tr590qtp",
								"Psi-nETH_staking":"terra1lws09x0slx892ux526d6atwwgdxnjg58uan8ph", 
							}
						});
	contracts[source][contract].to_string().replace(r#"""#, "")
}
pub fn get_mirrorprotocol_assets(source: &str, contract: &str) -> String {
	let contracts = json!({
 
									"mir": {
								      "symbol": "MIR",
								      "name": "Mirror",
								      "token": "terra15gwkyepfc6xgca5t5zefzwy42uts8l2m4g40k6",
								      "pair": "terra1amv303y8kzxuegvurh0gug2xe9wkgj65enq2ux",
								      "lpToken": "terra17gjf2zehfvnyjtdgua9p9ygquk6gukxe7ucgwh"
								    },
								    "mAAPL": {
								      "symbol": "mAAPL",
								      "name": "Apple Inc.",
								      "token": "terra1vxtwu4ehgzz77mnfwrntyrmgl64qjs75mpwqaz",
								      "pair": "terra1774f8rwx76k7ruy0gqnzq25wh7lmd72eg6eqp5",
								      "lpToken": "terra122asauhmv083p02rhgyp7jn7kmjjm4ksexjnks"
								    },
								    "mABNB": {
								      "symbol": "mABNB",
								      "name": "Airbnb Inc.",
								      "token": "terra1g4x2pzmkc9z3mseewxf758rllg08z3797xly0n",
								      "pair": "terra1gq7lq389w4dxqtkxj03wp0fvz0cemj0ek5wwmm",
								      "lpToken": "terra1jmauv302lfvpdfau5nhzy06q0j2f9te4hy2d07"
								    },
								    "mAMC": {
								      "symbol": "mAMC",
								      "name": "AMC Entertainment Holdings Inc.",
								      "token": "terra1qelfthdanju7wavc5tq0k5r0rhsyzyyrsn09qy",
								      "pair": "terra1uenpalqlmfaf4efgtqsvzpa3gh898d9h2a232g",
								      "lpToken": "terra1mtvslkm2tgsmh908dsfksnqu7r7lulh24a6knv"
								    },
								    "mAMZN": {
								      "symbol": "mAMZN",
								      "name": "Amazon.com, Inc.",
								      "token": "terra165nd2qmrtszehcfrntlplzern7zl4ahtlhd5t2",
								      "pair": "terra1vkvmvnmex90wanque26mjvay2mdtf0rz57fm6d",
								      "lpToken": "terra1q7m2qsj3nzlz5ng25z5q5w5qcqldclfe3ljup9"
								    },
								    "mBABA": {
								      "symbol": "mBABA",
								      "name": "Alibaba Group Holding Limited",
								      "token": "terra1w7zgkcyt7y4zpct9dw8mw362ywvdlydnum2awa",
								      "pair": "terra1afdz4l9vsqddwmjqxmel99atu4rwscpfjm4yfp",
								      "lpToken": "terra1stfeev27wdf7er2uja34gsmrv58yv397dlxmyn"
								    },
								    "m_btc": {
								      "symbol": "mBTC",
								      "name": "Bitcoin",
								      "token": "terra1rhhvx8nzfrx5fufkuft06q5marfkucdqwq5sjw",
								      "pair": "terra1prfcyujt9nsn5kfj5n925sfd737r2n8tk5lmpv",
								      "lpToken": "terra1d34edutzwcz6jgecgk26mpyynqh74j3emdsnq5"
								    },
								    "m_eth": {
								      "symbol": "mETH",
								      "name": "Ether",
								      "token": "terra1dk3g53js3034x4v5c3vavhj2738une880yu6kx",
								      "pair": "terra14fyt2g3umeatsr4j4g2rs8ca0jceu3k0mcs7ry",
								      "lpToken": "terra16auz7uhnuxrj2dzrynz2elthx5zpps5gs6tyln"
								    },
								    "mFB": {
								      "symbol": "mFB",
								      "name": "Facebook Inc.",
								      "token": "terra1mqsjugsugfprn3cvgxsrr8akkvdxv2pzc74us7",
								      "pair": "terra1yl2atgxw422qxahm02p364wtgu7gmeya237pcs",
								      "lpToken": "terra1jh2dh4g65hptsrwjv53nhsnkwlw8jdrxaxrca0"
								    },
								    "mGME": {
								      "symbol": "mGME",
								      "name": "GameStop Corp",
								      "token": "terra1m6j6j9gw728n82k78s0j9kq8l5p6ne0xcc820p",
								      "pair": "terra17eakdtane6d2y7y6v0s79drq7gnhzqan48kxw7",
								      "lpToken": "terra1azk43zydh3sdxelg3h4csv4a4uef7fmjy0hu20"
								    },
								    "mGOOGL": {
								      "symbol": "mGOOGL",
								      "name": "Alphabet Inc.",
								      "token": "terra1h8arz2k547uvmpxctuwush3jzc8fun4s96qgwt",
								      "pair": "terra1u56eamzkwzpm696hae4kl92jm6xxztar9uhkea",
								      "lpToken": "terra1falkl6jy4087h4z567y2l59defm9acmwcs70ts"
								    },
								    "mGS": {
								      "symbol": "mGS",
								      "name": "Goldman Sachs Group Inc.",
								      "token": "terra137drsu8gce5thf6jr5mxlfghw36rpljt3zj73v",
								      "pair": "terra108ukjf6ekezuc52t9keernlqxtmzpj4wf7rx0h",
								      "lpToken": "terra17smg3rl9vdpawwpe7ex4ea4xm6q038gp2chge5"
								    },
								    "mIAU": {
								      "symbol": "mIAU",
								      "name": "iShares Gold Trust",
								      "token": "terra15hp9pr8y4qsvqvxf3m4xeptlk7l8h60634gqec",
								      "pair": "terra1q2cg4sauyedt8syvarc8hcajw6u94ah40yp342",
								      "lpToken": "terra1jl4vkz3fllvj6fchnj2trrm9argtqxq6335ews"
								    },
								    "mMSFT": {
								      "symbol": "mMSFT",
								      "name": "Microsoft Corporation",
								      "token": "terra1227ppwxxj3jxz8cfgq00jgnxqcny7ryenvkwj6",
								      "pair": "terra10ypv4vq67ns54t5ur3krkx37th7j58paev0qhd",
								      "lpToken": "terra14uaqudeylx6tegamqmygh85lfq8qg2jmg7uucc"
								    },
								    "mNFLX": {
								      "symbol": "mNFLX",
								      "name": "Netflix, Inc.",
								      "token": "terra1jsxngqasf2zynj5kyh0tgq9mj3zksa5gk35j4k",
								      "pair": "terra1yppvuda72pvmxd727knemvzsuergtslj486rdq",
								      "lpToken": "terra1mwu3cqzvhygqg7vrsa6kfstgg9d6yzkgs6yy3t"
								    },
								    "mQQQ": {
								      "symbol": "mQQQ",
								      "name": "Invesco QQQ Trust",
								      "token": "terra1csk6tc7pdmpr782w527hwhez6gfv632tyf72cp",
								      "pair": "terra1dkc8075nv34k2fu6xn6wcgrqlewup2qtkr4ymu",
								      "lpToken": "terra16j09nh806vaql0wujw8ktmvdj7ph8h09ltjs2r"
								    },
								    "mSLV": {
								      "symbol": "mSLV",
								      "name": "iShares Silver Trust",
								      "token": "terra1kscs6uhrqwy6rx5kuw5lwpuqvm3t6j2d6uf2lp",
								      "pair": "terra1f6d9mhrsl5t6yxqnr4rgfusjlt3gfwxdveeyuy",
								      "lpToken": "terra178cf7xf4r9d3z03tj3pftewmhx0x2p77s0k6yh"
								    },
								    "m_tsla": {
								      "symbol": "mTSLA",
								      "name": "Tesla, Inc.",
								      "token": "terra14y5affaarufk3uscy2vr6pe6w6zqf2wpjzn5sh",
								      "pair": "terra1pdxyk2gkykaraynmrgjfq2uu7r9pf5v8x7k4xk",
								      "lpToken": "terra1ygazp9w7tx64rkx5wmevszu38y5cpg6h3fk86e"
								    },
								    "mTWTR": {
								      "symbol": "mTWTR",
								      "name": "Twitter, Inc.",
								      "token": "terra1cc3enj9qgchlrj34cnzhwuclc4vl2z3jl7tkqg",
								      "pair": "terra1ea9js3y4l7vy0h46k4e5r5ykkk08zc3fx7v4t8",
								      "lpToken": "terra1fc5a5gsxatjey9syq93c2n3xq90n06t60nkj6l"
								    },
								    "mUSO": {
								      "symbol": "mUSO",
								      "name": "United States Oil Fund, LP",
								      "token": "terra1lvmx8fsagy70tv0fhmfzdw9h6s3sy4prz38ugf",
								      "pair": "terra1zey9knmvs2frfrjnf4cfv4prc4ts3mrsefstrj",
								      "lpToken": "terra1utf3tm35qk6fkft7ltcnscwml737vfz7xghwn5"
								    },
								    "mVIXY": {
								      "symbol": "mVIXY",
								      "name": "ProShares VIX Short-Term Futures ETF",
								      "token": "terra1zp3a6q6q4953cz376906g5qfmxnlg77hx3te45",
								      "pair": "terra1yngadscckdtd68nzw5r5va36jccjmmasm7klpp",
								      "lpToken": "terra1cmrl4txa7cwd7cygpp4yzu7xu8g7c772els2y8"
								    },
								    "m_spy": {
								      "symbol": "mSPY",
								      "name": "SPDR S&P 500",
								      "token": "terra1aa00lpfexyycedfg5k2p60l9djcmw0ue5l8fhc",
								      "pair": "terra14hklnm2ssaexjwkcfhyyyzvpmhpwx6x6lpy39s",
								      "lpToken": "terra1jqqegd35rg2gjde54adpj3t6ecu0khfeaarzy9"
								    },
								    "mCOIN": {
								      "symbol": "mCOIN",
								      "name": "Coinbase Global, Inc.",
								      "token": "terra18wayjpyq28gd970qzgjfmsjj7dmgdk039duhph",
								      "pair": "terra1h7t2yq00rxs8a78nyrnhlvp0ewu8vnfnx5efsl",
								      "lpToken": "terra1ktckr8v7judrr6wkwv476pwsv8mht0zqzw2t0h"
								    }
								});
	contracts[source][contract].to_string().replace(r#"""#, "")
}