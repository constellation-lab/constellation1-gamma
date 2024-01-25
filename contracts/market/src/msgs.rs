#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use cosmwasm_std::{Coin,Uint128, Addr, Decimal,Timestamp};
use crate::state::{Bid, ContractParams};
//use schemars::JsonSchema;
#[cw_serde]
pub struct InstantiateMsg {
    pub option_address:String,
}

#[cw_serde]
pub enum ExecuteMsg {
    //Create a new option
    List {
        id: u64,
        price: Uint128,
        expires: u64,
    },
    RemoveList {
        id: u64,
    },
    UpdatePrice {
        id: u64,
        price: Uint128,
    },

    SetBid {
        id: u64,
        expires: u64,
    },
    RemoveBid {
        id: u64,
    },

    Buy {id: u64,},
    AcceptBid{id: u64,bidder: String,},
}  

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractParamsResponse)]
    ContractParams {},
    #[returns(ListItemsResponse)]
    ListItems{},
    #[returns(ListItemsResponse)]
    ListItemsPage{key: u64,amount: u64},
    #[returns(GetListItemByIdResponse)]
    GetListItemsByid{id:u64},
    #[returns(ListItemsResponse)]
    OwnerListItems{addr: String},
    #[returns(ListItemsResponse)]
    OwnerUnListItems{addr: String},

    #[returns(BidListRespose)]
    BidList{id: u64},

}

pub type ContractParamsResponse = ContractParams;
pub type ListItemsResponse = Vec<(u64,ListItemData)>;
pub type BidListRespose = Vec<(Addr,Bid)>;
pub type GetListItemByIdResponse = ListItemData;

#[cw_serde]
pub struct ListItemData{
    //option data
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Coin,
    pub counter_offer: Coin,
    pub expires: Timestamp,
    //List data
    pub price: Uint128,
    pub list_expires: Timestamp,
}
