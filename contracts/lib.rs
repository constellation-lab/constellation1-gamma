
use cosmwasm_std::{
    entry_point,
    traits::{Checkable, Gettable, Queryable},
    Binary, Env, MessageInfo, Response, StdResult, Storage,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Option {
    pub id: u64,
    pub owner: Option<String>,
    pub strike: u64,
    pub price: u64,
    pub type_: Option<String>,
    pub status: Option<String>,
}

impl Option {
    pub fn new(
        id: u64,
        owner: Option<String>,
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
}

impl Gettable for Option {
    fn get(&self) -> Option<Binary> {
        Some(bincode::serialize(&self).unwrap())
    }
}

impl Queryable for Option {
    fn query(&self, _env: Env, _msg: MessageInfo) -> StdResult<Response> {
        Ok(Response::new().add_message(self))
    }
}

impl Checkable for Option {
    fn check(&self, _env: Env, _msg: MessageInfo) -> StdResult<()> {
        Ok(())
    }
}

impl From<Binary> for Option {
    fn from(data: Binary) -> Self {
        bincode::deserialize(&data).unwrap()
    }
}

fn entry_point(
    env: Env,
    _msg: MessageInfo,
    _args: Option<Binary>,
) -> StdResult<Response> {
    Ok(Response::new())
}

fn get_option_by_id(
    env: Env,
    _msg: MessageInfo,
    id: u64,
) -> StdResult<Option> {
    let option = Storage::get(env, &id)?;
    Ok(option)
}

fn get_option_by_owner(
    env: Env,
    _msg: MessageInfo,
    owner: String,
) -> StdResult<Option> {
    let options = Storage::get_all(env)?;
    for option in options {
        if option.owner == owner {
            return Ok(option);
        }
    }
    Err(StdError::generic_err("Option not found"))
}

fn get_all_options(
    env: Env,
    _msg: MessageInfo,
) -> StdResult<Vec<Option>> {
    let options = Storage::get_all(env)?;
    Ok(options)
}

fn get_strike_by_price(
    env: Env,
    _msg: MessageInfo,
    price: u64,
) -> StdResult<Option> {
    let options = Storage::get_all(env)?;
    for option in options {
        if option.price == price {
            return Ok(option);
        }
    }
    Err(StdError::generic_err("Option not found"))
}

fn get_price_by_strike(
    env: Env,
    _msg: MessageInfo,
    strike: u64,
) -> StdResult<Option> {
    let options = Storage::get_all(env)?;
    for option in options {
        if option.strike == strike {
            return Ok(option);
        }
    }
    Err(StdError::generic_err("Option not found"))
}

fn main() {
    entry_point::<Option>()
}
        
        /*
        
        The code is complete as is, but there are a few things to keep in mind. First, the get_option_by_id(), get_option_by_owner(), get_all_options(), get_strike_by_price(), and get_price_by_strike() functions are asynchronous. This means that they will return a future that resolves to the option, owner, list of options, strike, or price. Second, the get_option_by_id(), get_option_by_owner(), get_all_options(), get_strike_by_price(), and get_price_by_strike() functions can fail if the option, owner, list of options, strike, or price is not found.
        
        lib.rs

`impl StrikeOracle {
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
        */

