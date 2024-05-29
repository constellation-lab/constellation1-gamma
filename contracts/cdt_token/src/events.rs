use cosmwasm_std::{Addr, Uint128};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Event, StdResult};

#[cw_serde]
pub enum CDTTokenEvent {
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
    Vote {
        proposal_id: u64,
        voter: Addr,
        vote: bool,
        voting_power: Uint128,
    },
}

impl CDTTokenEvent {
    pub fn into_event(self) -> StdResult<Event> {
        match self {
            CDTTokenEvent::Mint { recipient, amount } => Ok(Event::new("mint")
                .add_attribute("recipient", recipient.to_string())
                .add_attribute("amount", amount.to_string())),
            CDTTokenEvent::Burn { owner, amount } => Ok(Event::new("burn")
                .add_attribute("owner", owner.to_string())
                .add_attribute("amount", amount.to_string())),
            CDTTokenEvent::Transfer { from, to, amount } => Ok(Event::new("transfer")
                .add_attribute("from", from.to_string())
                .add_attribute("to", to.to_string())
                .add_attribute("amount", amount.to_string())),
            CDTTokenEvent::Vote {
                proposal_id,
                voter,
                vote,
                voting_power,
            } => Ok(Event::new("vote")
                .add_attribute("proposal_id", proposal_id.to_string())
                .add_attribute("voter", voter.to_string())
                .add_attribute("vote", vote.to_string())
                .add_attribute("voting_power", voting_power.to_string())),
        }
    }
}