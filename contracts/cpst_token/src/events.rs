use cosmwasm_std::{Event, Uint128};

pub fn mint_event(recipient: &str, amount: Uint128) -> Event {
    Event::new("mint")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string())
}

pub fn burn_event(owner: &str, amount: Uint128) -> Event {
    Event::new("burn")
        .add_attribute("owner", owner)
        .add_attribute("amount", amount.to_string())
}

pub fn transfer_event(from: &str, to: &str, amount: Uint128) -> Event {
    Event::new("transfer")
        .add_attribute("from", from)
        .add_attribute("to", to)
        .add_attribute("amount", amount.to_string())
}

pub fn stake_event(staker: &str, amount: Uint128) -> Event {
    Event::new("stake")
        .add_attribute("staker", staker)
        .add_attribute("amount", amount.to_string())
}