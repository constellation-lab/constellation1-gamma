use cosmwasm_std::{Addr, Coin, Timestamp};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map,Item};

#[allow(unused_imports)]
use crate::state::{Data as StateData, State as StateState};

#[cw_serde]
pub struct Data {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub counter_offer: Coin,
    //if one option is burned this will be set true, else is the false
    pub isBurned: bool,
    pub expires: Timestamp,
}

#[cw_serde]
pub struct State {
    pub owner: Addr,
    pub total_options_amount: u64,
}


pub const CONFIG_KEY: &str = "config";
pub const OPTION_LIST_KEY: &str = "option_list";
pub const APPROVE_LIST_KEY: &str = "approve_list";



pub const OPTION_LIST: Map<u64, Data> = Map::new(OPTION_LIST_KEY);

//(owner, spender)
pub const APPROVE_LIST: Map<(Addr,Addr),bool> = Map::new(APPROVE_LIST_KEY);

pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);



