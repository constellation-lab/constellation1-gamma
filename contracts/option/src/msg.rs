use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin};
use crate::state::{State,Data};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    //Create a new option
    Create {counter_offer: Vec<Coin>, time_stamp: u64},
    //Add the option to th market
    AddToMarket{id: u64, amount: u128, denom: String}, 
    //Remove from market
    RemoveFromMarket{id: u64},
    //buy option from market
    Buy {id: u64},
    //Update the sell price for a option
    UpdatePrice{id: u64,price: Vec<Coin>},
    // Owner can transfer to a new owner
    Transfer {id: u64,to: String },
    // Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {id: u64},
    // Burn will release collateral if expired
    Burn {id: u64},
    //Claim expier options collectal ayback the to the creator
    Claim{id: u64},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(OptionsResponse)]
    Options{},
    #[returns(OptionsResponse)]
    OptionsPage{key: u64,amount: u64},
    #[returns(GetOptionByIdResponse)]
    GetOptionByid{id:u64},
    #[returns(OptionsResponse)]
    MarketOptions{},
    #[returns(OptionsResponse)]
    MaketOptionsPagee{key: u64,amount: u64},
    #[returns(OptionsResponse)]
    CreateorOptions{addr: String},
    #[returns(OptionsResponse)]
    OwnerOptions{addr: String},

}

pub type ConfigResponse = State;
pub type OptionsResponse = Vec<(u64,Data)>;
pub type GetOptionByIdResponse = Data;
