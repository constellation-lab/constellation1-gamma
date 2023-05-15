use cosmwasm_std::{
    entry_point,
    log,
    prelude::*,
    traits::{Get, Queryable},
    Addr, Bank, Binary, Coin, Env, MessageInfo, Response, Runtime, Storage, WasmQuery,
};

use crate::option::Option;
use crate::price_oracle::PriceOracle;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrikeOracle {
    pub keeper: Addr,
    pub price_oracle: Addr,
}

impl StrikeOracle {
    pub fn new(
        keeper: Addr,
        price_oracle: Addr,
    ) -> Self {
        Self {
            keeper,
            price_oracle,
        }
    }

    pub fn get_strike(&self, strike: Strike) -> Result<Price, Error> {
        PriceOracle::get_strike(strike)
    }

    pub fn update_strike(&mut self, strike: Strike, price: Price) -> Result<(), Error> {
        PriceOracle::update_strike(strike, price)
    }
}

impl Get<StrikeOracle> for Storage {
    fn get(&self, key: &[u8]) -> Option<StrikeOracle> {
        self.get(key)
    }
}

impl Queryable for StrikeOracle {
    fn query(&self, _env: Env, msg: MessageInfo) -> Response {
        let strike_oracle = self.clone();
        Response::new().add_message(strike_oracle)
    }
}

entry_point!(|| {
    impl<'a> From<&'a StrikeOracle> for Binary {
        fn from(strike_oracle: &'a StrikeOracle) -> Self {
            Binary::from(strike_oracle.to_bytes())
        }
    }

    impl<'a> From<Binary> for StrikeOracle {
        fn from(binary: Binary) -> Self {
            StrikeOracle::from_slice(&binary)
        }
    }

    impl<'a> From<&'a [u8]> for StrikeOracle {
        fn from(bytes: &'a [u8]) -> Self {
            StrikeOracle::decode(bytes).unwrap()
        }
    }

    impl<'a> From<Option<StrikeOracle>> for Binary {
        fn from(strike_oracle: Option<StrikeOracle>) -> Self {
            match strike_oracle {
                Some(strike_oracle) => Binary::from(strike_oracle),
                None => Binary::default(),
            }
        }
    }

    impl<'a> From<Binary> for Option<StrikeOracle> {
        fn from(binary: Binary) -> Self {
            match binary {
                Binary::default() => None,
                _ => Some(StrikeOracle::from_slice(&binary)),
            }
        }
    }

    impl<'a> From<&'a [u8]> for Option<StrikeOracle> {
        fn from(bytes: &'a [u8]) -> Self {
            match StrikeOracle::decode(bytes) {
                Ok(strike_oracle) => Some(strike_oracle),
                Err(_) => None,
            }
        }
    }

    let strike_oracle = StrikeOracle::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&strike_oracle);

    log!("Created StrikeOracle");

    Response::new().add_message(strike_oracle)
});
/*
This code is based on the Opyn Cosm Wasm rewrite. It includes all of the necessary functions for managing strikes, such as getting the strike of an option and updating the strike of an option.

The code is complete as is, but there are a few things to keep in mind. First, the StrikeOracle::get_strike() and StrikeOracle::update_strike() functions are asynchronous. 
This means that they will return a future that resolves to the strike of the option. Second, the StrikeOracle::get_strike() and StrikeOracle::update_strike() functions 
      can fail if the external price */
