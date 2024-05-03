use cosmwasm_std::{Addr, Decimal, Uint128};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Event, StdResult, attr};

#[cw_serde]
pub enum OptionMarketplaceEvent {
    OptionListed {
        option_id: u64,
        price: Uint128,
        min_trade_amount: Uint128,
    },
    OptionBought {
        option_id: u64,
        buyer: Addr,
        amount: Uint128,
        total_price: Uint128,
    },
    OptionExecuted {
        option_id: u64,
    },
}

impl OptionMarketplaceEvent {
    pub fn into_event(self) -> StdResult<Event> {
        match self {
            OptionMarketplaceEvent::OptionListed {
                option_id,
                price,
                min_trade_amount,
            } => Ok(Event::new("option_listed")
                .add_attribute("option_id", option_id.to_string())
                .add_attribute("price", price.to_string())
                .add_attribute("min_trade_amount", min_trade_amount.to_string())),
            OptionMarketplaceEvent::OptionBought {
                option_id,
                buyer,
                amount,
                total_price,
            } => Ok(Event::new("option_bought")
                .add_attribute("option_id", option_id.to_string())
                .add_attribute("buyer", buyer.to_string())
                .add_attribute("amount", amount.to_string())
                .add_attribute("total_price", total_price.to_string())),
            OptionMarketplaceEvent::OptionExecuted { option_id } => {
                Ok(Event::new("option_executed").add_attribute("option_id", option_id.to_string()))
            }
        }
    }
}