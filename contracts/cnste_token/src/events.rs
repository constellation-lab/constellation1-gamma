use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Attribute, Event, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//#[cw_serde]
//pub enum CDTTokenEvent {

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CNSTETokenEvent {
    Mint {
        recipient: Addr,
        amount: Uint128,
    },
    Burn {
        owner: Addr,
        amount: Uint128,
    },
    Transfer {
        from: Addr,
        to: Addr,
        amount: Uint128,
    },
    Stake {
        staker: String,
        amount: Uint128,
    },

}

impl CNSTETokenEvent {
    pub fn into_event(self) -> StdResult<Event> {
        match self {
            CNSTETokenEvent::Mint { recipient, amount } => Ok(Event::new("mint")
                .add_attribute("recipient", recipient.to_string())
                .add_attribute("amount", amount.to_string())),
            CNSTETokenEvent::Burn { owner, amount } => Ok(Event::new("burn")
                .add_attribute("owner", owner.to_string())
                .add_attribute("amount", amount.to_string())),
            CNSTETokenEvent::Transfer { from, to, amount } => Ok(Event::new("transfer")
                .add_attribute("from", from.to_string())
                .add_attribute("to", to.to_string())
                .add_attribute("amount", amount.to_string())),
            CNSTETokenEvent::Stake { staker, amount } => Ok(Event::new("stake")
                .add_attribute("staker", staker)
                .add_attribute("amount", amount.to_string())),
        }
    }
}
