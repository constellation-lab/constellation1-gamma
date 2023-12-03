use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin, Timestamp, Decimal, Uint128};
use cw_storage_plus::{Map,Item};
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::state::{Data as StateData, State as StateState};

#[cw_serde]
pub enum OptionStatus {
    Active,
    Expired,
}

#[cw_serde]
pub enum AcceptanceStatusResponse {
  BidAccepted,
  OfferAccepted,
  NoBidOrOffer  
}

#[cw_serde]
pub struct Data {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub onsale: bool,
    pub price: Vec<Coin>,
    pub expires: Timestamp,
    pub highest_bidder: Option<Addr>,
    pub best_offer: Option<Addr>,
    pub bid_history: Option<Vec<Bid>>, 
    pub status: OptionStatus,
    pub parameters: Option<HashMap<String, String>>,
    pub exercise_conditions: Option<Vec<String>>,
    pub history: Vec<String>,
    pub risk_metrics: Option<HashMap<String, Decimal>>,
    pub pool_share: Uint128, 
}

#[cw_serde]
pub struct State {
    pub creator: Addr,
    pub total_options_num: u64,
    pub paused: bool,
    pub oracle: Option<Addr>,
}

#[cw_serde]
pub struct BidOrOfferResponse {
  pub bidder: Option<Addr>,
  pub offeror: Option<Addr>,
}

#[cw_serde]
pub struct Bid {
    pub bidder: Addr, 
    pub amount: Vec<Coin>   
}

#[cw_serde]
pub struct Lock {
    expiration: u64,
    amount: Vec<Coin>   
}

#[cw_serde]
pub struct DiscountCriteria {
    pub criteria: HashMap<String, HashMap<String, String>>,
}

#[cw_serde]
pub struct AmmPoolInfo {
    id: u64,
    reserves: [Coin; 2], // One token, one stablecoin  
    fee: Decimal 
}
  
#[cw_serde]
pub struct LockInfo {
    owner: Addr,
    amount: Coin,
    expiration: u64  
}

#[cw_serde]
pub struct Integrate {
    option_id: u64,
    feed_url: String
}

#[cw_serde]
pub struct PartialExecutionResponse {
    pub collateral_amount: Uint128,
    pub counter_offer_amount: Uint128,
}

#[cw_serde]
pub struct MarketOptionPriceResponse {
    pub price: Vec<Coin>,
}

#[cw_serde]
pub struct CollateralUsageResponse {
    pub collateral_usage_info: CollateralUsageInfo,
}

#[cw_serde]
pub struct OptionStatusResponse {
    pub status: OptionStatus,
}
#[cw_serde]
pub struct CollateralUsageInfo {
    pub in_use: bool,
    pub used_amount: Uint128,
    // Add other fields as needed...
}

#[cw_serde]
pub struct AMMPoolDetailsResponse {
    pub amm_pool_details: Vec<String>,
    // Add other fields as needed...
}

#[cw_serde]
pub struct YieldFarmingInfoResponse {
    pub yield_farming_info: YieldFarmingInfo,
}

#[cw_serde]
pub struct YieldFarmingInfo {
    pub is_wrapped: bool,
    pub wrapped_amount: Uint128,
    // Add other fields as needed...
}

#[cw_serde]
pub struct DataFeedIntegrationResponse {
    pub data_feed_integration_info: DataFeedIntegrationInfo,
}

#[cw_serde]
pub struct DataFeedIntegrationInfo {
    pub is_integrated: bool,
    pub data_feed_url: String,
    // Add other fields as needed...
}

#[cw_serde]
pub struct DiscountCriteriaResponse {
    pub discount_criteria: DiscountCriteria,
}

pub const CONFIG_KEY: &str = "config";
pub const OPTION_LIST_KEY: &str = "option_list";
pub const OWN_OPTIONS_KEY: &str = "own_options";
pub const CREATE_OPTIONS_KEY: &str = "create_options";
pub const MARKET_LIST_KEY: &str = "market_options";


pub const OPTION_LIST: Map<u64, Data> = Map::new(OPTION_LIST_KEY);
pub const CREATOR_LIST: Map<(Addr, u64),Data> = Map::new(CREATE_OPTIONS_KEY);
pub const OWNER_LIST: Map<(Addr, u64),Data> = Map::new(OWN_OPTIONS_KEY);
pub const MARKET_LIST: Map<u64,Data> = Map::new(MARKET_LIST_KEY);

pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);

pub const AMM_POOLS: Map<u64, AmmPoolInfo> = Map::new("amm_pools");
pub const LOCKS: Map<&Addr, LockInfo> = Map::new("locks");  
pub const DISCOUNTS: Map<&Addr, DiscountCriteria> = Map::new("discounts");




/*pub mod state {

    pub use super::{Data, State};  
    pub use super::{BidOrOfferResponse, Bid, Lock, DiscountCriteria, AmmPoolInfo, LockInfo, Integrate}; 
    
    //pub use super::ACCEPTANCE_STATUS;

    // Export any other desired types
    pub use super:: CONFIG_KEY;
    pub use super:: OPTION_LIST_KEY;
    pub use super:: OWN_OPTIONS_KEY;
    pub use super:: CREATE_OPTIONS_KEY;
    pub use super:: MARKET_LIST_KEY;


    pub use super:: OPTION_LIST;
    pub use super:: CREATOR_LIST;
    pub use super:: OWNER_LIST;
    pub use super:: MARKET_LIST;

    pub use super:: CONFIG;

    pub use super:: AMM_POOLS;
    pub use super:: LOCKS;
    pub use super:: DISCOUNTS;
    

  }*/
