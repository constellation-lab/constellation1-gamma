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

fn constructor(
    env: Env,
    _msg: MessageInfo,
    _args: Option<Binary>,
) -> StdResult<Response> {
    Ok(Response::new())
}

fn initialize(
    env: Env,
    _msg: MessageInfo,
    _args: Option<Binary>,
) -> StdResult<Response> {
    Ok(Response::new())
}

fn update_option_status(
    env: Env,
    msg: MessageInfo,
    args: Option<Binary>,
) -> StdResult<Response> {
    let option = Option::from(args.unwrap());

    // Check if the sender is the owner of the option.
    if option.owner != msg.sender() {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Update the option status.
    match option.status {
        Some("open") => {
            Storage::set(env, &option.id, &option);
            Ok(Response::new())
        }
        Some("closed") => {
            // TODO: Refund the option price to the owner.
            refund_option_price(env, msg, args);
            Storage::set(env, &option.id, &option);
            Ok(Response::new())
        }
        Some("exercised") => {
            // TODO: Pay out the option price to the owner.
            pay_out_option_price(env, msg, args);
            Storage::set(env, &option.id, &option);
            Ok(Response::new())
        }
        _ => Err(StdError::generic_err("Invalid option status")),
    }
}


fn mint_option(
    env: Env,
    msg: MessageInfo,
    args: Option<Binary>,
) -> StdResult<Response> {
    let option = Option::from(args.unwrap());

    // Check if the sender is the owner of the option.
    if option.owner != msg.sender() {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Mint the option.
    Storage::insert(&option);
    Ok(Response::new())
}

fn burn_option(
    env: Env,
    msg: MessageInfo,
    args: Option<Binary>,
) -> StdResult<Response> {
    let option = Option::from(args.unwrap());

    // Check if the sender is the owner of the option.
    if option.owner != msg.sender() {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Burn the option.
    Storage::remove(env, &option.id);
    Ok(Response::new())
}

fn pay_out_option_price(
    env: Env,
    msg: MessageInfo,
    args: Option<Binary>,
) -> StdResult<Response> {
    let option = Option::from(args.unwrap());

    // Check if the sender is the owner of the option.
    if option.owner != msg.sender() {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Pay out the option price to the owner.
    let amount = option.price;
    env.transfer(option.owner, amount);

    // Update the option status.
    option.status = "paid";
    Storage::set(env, &option.id, &option);

    Ok(Response::new())
}
        
        
fn refund_option_price(
    env: Env,
    msg: MessageInfo,
    args: Option<Binary>,
) -> StdResult<Response> {
    let option = Option::from(args.unwrap());

    // Check if the sender is the owner of the option.
    if option.owner != msg.sender() {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Refund the option price to the owner.
    let amount = option.price;
    env.transfer(option.owner, amount);

    // Update the option status.
    option.status = "refunded";
    Storage::set(env, &option.id, &option);

    Ok(Response::new())
}

/*
        
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

fn update_option_status(
    env: Env,
    _msg: MessageInfo,
    id: u64,
    status: Option<String>,
) -> StdResult<()> {
    let option = Storage::get(env, &id)?;
    option.status = status;
    Storage::insert(&option);
    log!("Updated option status");
    Response::new().add_message(option)
}

fn mint_option(
    env: Env,
    _msg: MessageInfo,
    option: Option,
) -> StdResult<()> {
    Storage::insert(&option);
    log!("Minted option");
    Response::new().add_message(option)
}

fn burn_option(
    env: Env,
    _msg: MessageInfo,
    id: u64,
) -> StdResult<()> {
    Storage::remove(env, &id);
    log!("Burned option");
    Response::new()
}

fn exercise_option(
    env: Env,
    _msg: MessageInfo,
    id: u64,
) -> StdResult<()> {
    // TODO: Implement exercise_option()
    log!("Exercised option");
    Response::new()
}

fn refund_option(
    env: Env,
    _msg: MessageInfo,
    id: u64,
) -> StdResult<()> {
    // TODO: Implement refund_option()
    log!("Refunded option");
    Response::new()
}

fn main() {
    entry_point::<Option>()
}

        
        
        
        
        */



//The Keeper contract is responsible for managing the options on the OpynFinance Cosm WASM rewrite. It stores the following information:

//The address of the contract's owner
//The address of the contract's price oracle
//The address of the contract's strike oracle
//The contract also provides the following methods:

//get_option(option_id: OptionId) -> Option<Option>: This method returns the option with the given id.
//create_option(option: Option) -> Result<(), Error>: This method creates a new option.
//update_option(option_id: OptionId, option: Option) -> Result<(), Error>: This method updates an existing option.
//delete_option(option_id: OptionId) -> Result<(), Error>: This method deletes an existing option.
//get_price(strike: Strike) -> Result<Price, Error>: This method returns the price of the given strike.
//get_strike(price: Price) -> Result<Strike, Error>: This method returns the strike of the given price.
//The Keeper contract is a critical part of the OpynFinance Cosm WASM rewrite. 
//It allows users to easily manage their options and ensures that the options are always priced accurately.


/*
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
pub struct Keeper {
    pub calliope: Addr,
    pub price_oracle: Addr,
    pub strike_oracle: Addr,
}

impl Keeper {
    pub fn new(
        calliope: Addr,
        price_oracle: Addr,
        strike_oracle: Addr,
    ) -> Self {
        Self {
            calliope,
            price_oracle,
            strike_oracle,
        }
    }

    pub fn get_option(&self, option_id: OptionId) -> Option<Option> {
        let option = Option::get(option_id);
        Some(option)
    }

    pub fn create_option(&mut self, option: Option) -> Result<(), Error> {
        Option::insert(option)?;
        Ok(())
    }

    pub fn update_option(&mut self, option_id: OptionId, option: Option) -> Result<(), Error> {
        Option::update(option_id, option)?;
        Ok(())
    }

    pub fn delete_option(&mut self, option_id: OptionId) -> Result<(), Error> {
        Option::remove(option_id)?;
        Ok(())
    }

    pub fn get_price(&self, strike: Strike) -> Result<Price, Error> {
        PriceOracle::get_price(strike)
    }

    pub fn get_strike(&self, price: Price) -> Result<Strike, Error> {
        StrikeOracle::get_strike(price)
    }
}

impl Get<Keeper> for Storage {
    fn get(&self, key: &[u8]) -> Option<Keeper> {
        self.get(key)
    }
}

impl Queryable for Keeper {
    fn query(&self, _env: Env, msg: MessageInfo) -> Response {
        let keeper = self.clone();
        Response::new().add_message(keeper)
    }
}

entry_point!(|| {
    impl<'a> From<&'a Keeper> for Binary {
        fn from(keeper: &'a Keeper) -> Self {
            Binary::from(keeper.to_bytes())
        }
    }

    impl<'a> From<Binary> for Keeper {
        fn from(binary: Binary) -> Self {
            Keeper::from_slice(&binary)
        }
    }

    impl<'a> From<&'a [u8]> for Keeper {
        fn from(bytes: &'a [u8]) -> Self {
            Keeper::decode(bytes).unwrap()
        }
    }

    impl<'a> From<Option<Keeper>> for Binary {
        fn from(keeper: Option<Keeper>) -> Self {
            match keeper {
                Some(keeper) => Binary::from(keeper),
                None => Binary::default(),
            }
        }
    }

    impl<'a> From<Binary> for Option<Keeper> {
        fn from(binary: Binary) -> Self {
            match binary {
                Binary::default() => None,
                _ => Some(Keeper::from_slice(&binary)),
            }
        }
    }

    impl<'a> From<&'a [u8]> for Option<Keeper> {
        fn from(bytes: &'a [u8]) -> Self {
            match Keeper::decode(bytes) {
                Ok(keeper) => Some(keeper),
                Err(_) => None,
            }
        }
    }

    let keeper = Keeper::new(
        env::sender(),
        env::message().
      //continue
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&keeper);

    log!("Created Keeper");

    Response::new().add_message(keeper)
});*/

//This code creates a new Keeper contract and stores it in the contract's storage. The contract is then initialized with the following information:

//The address of the contract's owner
//The address of the contract's price oracle
//The address of the contract's strike oracle
//The contract is then logged and a response is returned.

