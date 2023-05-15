use cosmwasm_std::{
    entry_point,
    log,
    prelude::*,
    traits::{Get, Queryable},
    Addr, Bank, Binary, Coin, Env, MessageInfo, Response, Runtime, Storage, WasmQuery,
};

use crate::option::Option;
use crate::price_oracle::PriceOracle;
use crate::strike_oracle::StrikeOracle;

#[test]
fn test_get_option() {
    let option = Option::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&option);

    assert_eq!(
        Option::get(option.id),
        Some(option),
        "Expected to get the option"
    );
}

#[test]
fn test_update_option() {
    let option = Option::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&option);

    let new_option = Option::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::update(option.id, new_option);

    assert_eq!(
        Option::get(option.id),
        Some(new_option),
        "Expected to update the option"
    );
}

#[test]
fn test_get_price() {
    let price = Price::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::insert(&price);

    assert_eq!(
        PriceOracle::get_price(price.strike),
        Some(price),
        "Expected to get the price"
    );
}

#[test]
fn test_update_price() {
    let price = Price::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::insert(&price);

    let new_price = Price::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::update(price.strike, new_price);

    assert_eq!(
        PriceOracle::get_price(price.strike),
        Some(new_price),
        "Expected to update the price"
    );
}

#[test]
fn test_get_strike() {
    let strike = Strike::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::insert(&strike);

    assert_eq!(
        StrikeOracle::get_strike(strike.strike),
        Some(strike),
        "Expected to get the strike"
    );
}

#[test]
fn test_update_strike() {
    let strike = Strike::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::insert(&strike);

    let new_strike = Strike::new(
        //env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
       
    );

    Storage::update(strike.strike, new_strike);

    assert_eq!(
        StrikeOracle::get_strike(strike.strike),
        Some(new_strike),
        "Expected to update the strike"
    );
}
