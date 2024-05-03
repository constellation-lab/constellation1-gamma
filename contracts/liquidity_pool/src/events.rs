use cosmwasm_std::{attr, Attribute, StdResult, Event, Addr, to_json_string};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Response};
// Events implementation
#[cw_serde]
pub enum LiquidityPoolEvent {
Deposit {
lp_address: Addr,
assets: Vec<String>,
lp_tokens: String,
},
Withdraw {
lp_address: Addr,
assets: Vec<String>,
lp_tokens: String,
},
OptionMinted {
option_id: u64,
collateral: String,
counter_offer: String,
expiration: u64,
},
PremiumDistributed {
option_id: u64,
premium: String,
},
}
impl LiquidityPoolEvent {
    pub fn emit_deposit(
    deps: Deps,
    lp_address: Addr,
    assets: Vec<String>,
    lp_tokens: String,
    ) -> StdResult<()> {
    let attrs = vec![
    attr("action", "deposit"),
    attr("lp_address", lp_address.to_string()),
    attr("assets", format!("{:?}", assets)),
    attr("lp_tokens", lp_tokens.clone()),
    ];
    Self::log_event(
    deps,
    LiquidityPoolEvent::Deposit {
    lp_address,
    assets,
    lp_tokens,
    },
    "deposit",
    attrs,
    )
    }

    pub fn emit_withdraw(
        deps: Deps,
        lp_address: Addr,
        assets: Vec<String>,
        lp_tokens: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "withdraw"),
            attr("lp_address", lp_address.to_string()),
            attr("assets", format!("{:?}", assets)),
            attr("lp_tokens", lp_tokens.clone()),
        ];
        Self::log_event(
            deps,
            LiquidityPoolEvent::Withdraw {
                lp_address,
                assets,
                lp_tokens,
            },
            "withdraw",
            attrs,
        )
    }
    
    pub fn emit_option_minted(
        deps: Deps,
        option_id: u64,
        collateral: String,
        counter_offer: String,
        expiration: u64,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_minted"),
            attr("option_id", option_id.to_string()),
            attr("collateral", collateral.clone()),
            attr("counter_offer", counter_offer.clone()),
            attr("expiration", expiration.to_string()),
        ];
        Self::log_event(
            deps,
            LiquidityPoolEvent::OptionMinted {
                option_id,
                collateral,
                counter_offer,
                expiration,
            },
            "option_minted",
            attrs,
        )
    }
    
    pub fn emit_premium_distributed(deps: Deps, option_id: u64, premium: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "premium_distributed"),
            attr("option_id", option_id.to_string()),
            attr("premium", premium.clone()),
        ];
        Self::log_event(
            deps,
            LiquidityPoolEvent::PremiumDistributed { option_id, premium },
            "premium_distributed",
            attrs,
        )
    }
    

fn log_event(
    _deps: Deps,
    event: LiquidityPoolEvent,
    event_type: &str,
    attrs: Vec<Attribute>,
) -> StdResult<()> {
    // Serialize the event data into a string
    let event_data_str = match to_json_string(&event) {
        Ok(json) => json,
        Err(err) => {
            let msg = format!("{}", err);
            return Err(cosmwasm_std::StdError::generic_err(msg));
        }
    };

    // Add the serialized data as an attribute
    let mut custom_attrs = attrs;
    custom_attrs.push(Attribute {
        key: "data".to_string(),
        value: event_data_str,
    });

    // Create a new event with the given type and attributes
    let custom_event = Event::new(event_type).add_attributes(custom_attrs);
    let custom_event_name = format!("LiquidityPoolEvent_{}", event_type);

    // Create a response and add the custom event to it
    let response: Response<()> = Response::new().add_event(custom_event);

    // Add default "wasm" event with additional attributes if needed
    response.add_attribute("action", custom_event_name);

    // Return the response
    Ok(())
}
}