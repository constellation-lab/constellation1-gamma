
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};

use crate::state::LiquidityProviderInfo;
#[cw_serde]
pub struct InstantiateMsg {}
#[cw_serde]
pub enum ExecuteMsg {
Deposit { assets: Vec<Coin> },
Withdraw { lp_tokens: Uint128 },
MintOption {
collateral: Coin,
counter_offer: Coin,
expiration: u64,
},
DistributePremium { option_id: u64 },
}
#[cw_serde]
pub enum QueryMsg {
GetPool {},
GetLiquidityProviderInfo { address: String },
}




