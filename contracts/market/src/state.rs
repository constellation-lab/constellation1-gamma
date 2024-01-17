use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Map,Item};


/// Represents an ask on the marketplace
#[cw_serde]
pub struct ListItem {
    pub seller: Addr,
    pub price: Uint128,
    pub expires_at: Timestamp,
}

/// Represents a bid (offer) on the marketplace
#[cw_serde]
pub struct Bid {
    pub price: Uint128,
    pub expires_at: Timestamp,
}

#[cw_serde]
pub struct ContractParams{
    pub option_address: Addr,
    pub onwer_address: Addr,
}

pub const CONTRACT_PARAMS_KEY: &str = "contract_params";
pub const LISTITEM_LIST_KEY: &str = "market_list";
pub const BID_LIST_KEY: &str = "bid_list";


//token id
pub const LISTITEM_LIST: Map<u64, ListItem> = Map::new(LISTITEM_LIST_KEY);

//(token_id, bidder)
pub const BID_LIST: Map<(u64,Addr),Bid> = Map::new(BID_LIST_KEY);

pub const CONTRACT_PARAMS: Item<ContractParams> = Item::new(CONTRACT_PARAMS_KEY);

