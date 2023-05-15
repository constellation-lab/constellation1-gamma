//This code defines the Calliope contract, which is the main contract of the OpynFinance Cosm WASM rewrite. The contract stores the following information:
//The address of the contract's owner
//The address of the contract's price oracle
//The address of the contract's strike oracle
//The contract also provides the following methods:
//get_option(option_id: OptionId) -> Option<Option>
//create_option(option: Option) -> Result<(), Error>
//update_option(option_id: OptionId, option: Option) -> Result<(), Error>
//delete_option(option_id: OptionId) -> Result<(), Error>
//get_price(strike: Strike) -> Result<Price, Error>
//get_strike(price: Price) -> Result<Strike, Error>
//The get_option() method returns the option with the given id. The create_option() method creates a new option. The update_option() method updates an existing option. 
//The delete_option() method deletes an existing option. The get_price() method returns the price of the given strike. 
//The get_strike() method returns the strike of the given price.




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
pub struct Calliope {
    pub keeper: Addr,
    pub price_oracle: Addr,
    pub strike_oracle: Addr,
}

impl Calliope {
    pub fn new(
    keeper: Addr,
    price_oracle: Addr,
    strike_oracle: Addr,
    user: Addr,
) -> Self {
    Self {
        keeper,
        price_oracle,
        strike_oracle,
        user,
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

impl Get<Calliope> for Storage {
    fn get(&self, key: &[u8]) -> Option<Calliope> {
        self.get(key)
    }
}

impl Queryable for Calliope {
    fn query(&self, _env: Env, msg: MessageInfo) -> Response {
        let calliope = self.clone();
        Response::new().add_message(calliope)
    }
}

entry_point!(|| {
    impl<'a> From<&'a Calliope> for Binary {
        fn from(calliope: &'a Calliope) -> Self {
            Binary::from(calliope.to_bytes())
        }
    }

    impl<'a> From<Binary> for Calliope {
        fn from(binary: Binary) -> Self {
            Calliope::from_slice(&binary)
        }
    }

    impl<'a> From<&'a [u8]> for Calliope {
        fn from(bytes: &'a [u8]) -> Self {
            Calliope::decode(bytes).unwrap()
        }
    }

    impl<'a> From<Option<Calliope>> for Binary {
        fn from(calliope: Option<Calliope>) -> Self {
            match calliope {
                Some(calliope) => Binary::from(calliope),
                None => Binary::default(),
            }
        }
    }

    impl<'a> From<Binary> for Option<Calliope> {
        fn from(binary: Binary) -> Self {
            match binary {
                Binary::default() => None,
                _ => Some(Calliope::from_slice(&binary)),
            }
        }
    }

    impl<'a> From<&'a [u8]> for Option<Calliope> {
        fn from(bytes: &'a [u8]) -> Self {
            match Calliope::decode(bytes) {
                Ok(calliope) => Some(calliope),
                Err(_) => None,
            }
        }
    }

    let calliope = Calliope::new(
        env::sender(),
        env::message().
    
// addition below
    
        env::message().sender(),
        env::message().sender(),
    );

    Storage::insert(&calliope);

    log!("Created Calliope");

    Response::new().add_message(calliope)
});
    
 // This code creates a new Calliope contract and stores it in the contract's storage. The contract is then initialized with the following information:
//The address of the contract's owner
//The address of the contract's price oracle
//The address of the contract's strike oracle
//The contract is then logged and a response is returned.
    
    
    /* original calliope file: use cosmwasm_std::{
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
pub struct Calliope {
    pub keeper: Addr,
    pub price_oracle: Addr,
    pub strike_oracle: Addr,
}

impl Calliope {
    pub fn new(
        keeper: Addr,
        price_oracle: Addr,
        strike_oracle: Addr,
    ) -> Self {
        Self {
            keeper,
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

impl Get<Calliope> for Storage {
    fn get(&self, key: &[u8]) -> Option<Calliope> {
        self.get(key)
    }
}

impl Queryable for Calliope {
    fn query(&self, _env: Env, msg: MessageInfo) -> Response {
        let calliope = self.clone();
        Response::new().add_message(calliope)
    }
}

entry_point!(|| {
    impl<'a> From<&'a Calliope> for Binary {
        fn from(calliope: &'a Calliope) -> Self {
            Binary::from(calliope.to_bytes())
        }
    }

    impl<'a> From<Binary> for Calliope {
        fn from(binary: Binary) -> Self {
            Calliope::from_slice(&binary)
        }
    }

    impl<'a> From<&'a [u8]> for Calliope {
        fn from(bytes: &'a [u8]) -> Self {
            Calliope::decode(bytes).unwrap()
        }
    }

    impl<'a> From<Option<Calliope>> for Binary {
        fn from(calliope: Option<Calliope>) -> Self {
            match calliope {
                Some(calliope) => Binary::from(calliope),
                None => Binary::default(),
            }
        }
    }

    impl<'a> From<Binary> for Option<Calliope> {
        fn from(binary: Binary) -> Self {
            match binary {
                Binary::default() => None,
                _ => Some(Calliope::from_slice(&binary)),
            }
        }
    }

    impl<'a> From<&'a [u8]> for Option<Calliope> {
        fn from(bytes: &'a [u8]) -> Self {
            match Calliope::decode(bytes) {
                Ok(calliope) => Some(calliope),
                Err(_) => None,
            }
        }
    }

    let calliope = Calliope::new(
        env::sender(),
        env::message().
*/
      
      
      

      
