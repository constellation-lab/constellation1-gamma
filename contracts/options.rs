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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    pub id: u64,
    pub owner: Addr,
    pub strike: u64,
    pub price: u64,
    pub type_: Option<String>,
    pub status: Option<String>,
}

impl Options {
    pub fn new(
        id: u64,
        owner: Addr,
        strike: u64,
        price: u64,
        type_: Option<String>,
        status: Option<String>,
    ) -> Self {
        Self {
            id,
            owner,
            strike,
            price,
            type_,
            status,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_owner(&self) -> Addr {
        self.owner
    }

    pub fn get_strike(&self) -> u64 {
        self.strike
    }

    pub fn get_price(&self) -> u64 {
        self.price
    }

    pub fn get_type(&self) -> Option<String> {
        self.type_
    }

    pub fn get_status(&self) -> Option<String> {
        self.status
    }
}

impl Get<Options> for Storage {
    fn get(&self, key: &[u8]) -> Option<Options> {
        self.get(key)
    }
}

impl Queryable for Options {
    fn query(&self, _env: Env, msg: MessageInfo) -> Response {
        let options = self.clone();
        Response::new().add_message(options)
    }
}

entry_point!(|| {
    impl<'a> From<&'a Options> for Binary {
        fn from(options: &'a Options) -> Self {
            Binary::from(options.to_bytes())
        }
    }

    impl<'a> From<Binary> for Options {
        fn from(binary: Binary) -> Self {
            Options::from_slice(&binary)
        }
    }

    impl<'a> From<&'a [u8]> for Options {
        fn from(bytes: &'a [u8]) -> Self {
            Options::decode(bytes).unwrap()
        }
    }

    impl<'a> From<Option<Options>> for Binary {
        fn from(options: Option<Options>) -> Self {
            match options {
                Some(options) => Binary::from(options),
                None => Binary::default(),
            }
        }
    }

    impl<'a> From<Binary> for Option<Options> {
        fn from(binary: Binary) -> Self {
            match binary {
                Binary::default() => None,
                _ => Some(Options::from_slice(&binary)),
            }
        }
    }

    impl<'a> From<&'a [u8]> for Option<Options> {
        fn from(bytes: &'a [u8]) -> Self {
            match Options::decode(bytes) {
                Ok(options) => Some(options),
                Err(_) => None,
            }
        }
    }

    let options = Options::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&options);

    log!("Created Options");

    Response::new().add_message(options)
});
