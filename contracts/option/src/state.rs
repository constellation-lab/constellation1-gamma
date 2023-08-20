use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin, Timestamp};
use cw_storage_plus::{Map,Item};

#[cw_serde]
pub struct Data {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>,
    pub counter_offer: Vec<Coin>,
    pub onsale: bool,
    pub price: Vec<Coin>,
    pub expires: Timestamp,
}

#[cw_serde]
pub struct State {
    pub creator: Addr,
    pub total_options_num: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const OPTION_LIST_KEY: &str = "option_list";
pub const OWN_OPTIONS_KEY: &str = "own_options";
pub const CREATE_OPTIONS_KEY: &str = "create_options";



pub const OPTION_LIST: Map<u64, Data> = Map::new(OPTION_LIST_KEY);
pub const CREATOR_LIST: Map<(Addr, u64),Data> = Map::new(CREATE_OPTIONS_KEY);
pub const OWNER_LIST: Map<(Addr, u64),Data> = Map::new(OWN_OPTIONS_KEY);
pub const MARKET_LIST: Map<u64,Data> = Map::new(OPTION_LIST_KEY);


pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);