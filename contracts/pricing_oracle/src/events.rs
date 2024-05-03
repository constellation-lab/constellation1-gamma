use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, attr, Attribute, Event, StdResult, Uint128, to_json_string, Response};

// Events implementation
#[cw_serde]
pub enum PricingOracleEvent {
OptionPriceCalculated {
option_id: u64,
price: String,
},
PricingParametersAdjusted {
volatility_multiplier: String,
},
}
impl PricingOracleEvent {
pub fn emit_option_price_calculated(deps: Deps, option_id: u64, price: String) -> StdResult<()> {
let attrs = vec![
attr("action", "option_price_calculated"),
attr("option_id", option_id.to_string()),
attr("price", price.clone()),
];
Self::log_event(
deps,
PricingOracleEvent::OptionPriceCalculated { option_id, price },
"option_price_calculated",
attrs,
)
}

pub fn emit_pricing_parameters_adjusted(deps: Deps, volatility_multiplier: String) -> StdResult<()> {
    let attrs = vec![
        attr("action", "pricing_parameters_adjusted"),
        attr("volatility_multiplier", volatility_multiplier.clone()),
    ];
    Self::log_event(
        deps,
        PricingOracleEvent::PricingParametersAdjusted {
            volatility_multiplier,
        },
        "pricing_parameters_adjusted",
        attrs,
    )
}

fn log_event(
    _deps: Deps,
    event: PricingOracleEvent,
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
    let custom_event_name = format!("PricingOracleEvent_{}", event_type);

    // Create a response and add the custom event to it
    //let response = cosmwasm_std::Response::new().add_event(custom_event);
    let response: Response = cosmwasm_std::Response::new().add_event(custom_event);
    
    //let response: Response<T> = cosmwasm_std::Response::new().add_event(custom_event);

    // Add default "wasm" event with additional attributes if needed
    response.add_attribute("action", custom_event_name);

    // Return the response
    Ok(())
}

}