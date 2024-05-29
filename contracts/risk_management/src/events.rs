use cosmwasm_std::{attr, Attribute, StdResult, Event, Addr, to_json_string, Response};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Deps;
// Events implementation

#[cw_serde]
pub enum RiskManagementEvent {
    PositionLimitSet {
        option_pair: String,
        max_position: String,
    },
    CircuitBreakerSet {
        option_pair: String,
        price_threshold: String,
        triggered: bool,
    },
    RiskMitigationStrategyExecuted {},
    DynamicPricingAdjusted {
        option_pair: String,
        adjustment_factor: String,
    },
    PositionClosed {
        ption_pair: String,
        amount: String,
    },
    VolatilityMultiplierAdjusted {
        volatility_multiplier: String,
    },
}

impl RiskManagementEvent {
    pub fn emit_position_limit_set(deps: Deps, option_pair: String, max_position: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "position_limit_set"),
            attr("option_pair", option_pair.clone()),
            attr("max_position", max_position.clone()),
        ];
    Self::log_event(
        deps,
        RiskManagementEvent::PositionLimitSet {
            option_pair,
            max_position,
        },
        "position_limit_set",
        attrs,)
    }

fn emit_circuit_breaker_set(
    deps: Deps,
    option_pair: String,
    price_threshold: String,
    triggered: bool,
) -> StdResult<()> {
    let attrs = vec![
        attr("action", "circuit_breaker_set"),
        attr("option_pair", option_pair.clone()),
        attr("price_threshold", price_threshold.clone()),
        attr("triggered", triggered.to_string()),
    ];
    Self::log_event(
        deps,
        RiskManagementEvent::CircuitBreakerSet {
            option_pair,
            price_threshold,
            triggered,
        },
        "circuit_breaker_set",
        attrs,
    )
}

pub fn emit_risk_mitigation_strategy_executed(deps: Deps) -> StdResult<()> {
    let attrs = vec![attr("action", "risk_mitigation_strategy_executed")];
    Self::log_event(
        deps,
        RiskManagementEvent::RiskMitigationStrategyExecuted {},
        "risk_mitigation_strategy_executed",
        attrs,
    )
}

pub fn emit_dynamic_pricing_adjusted(
    deps: Deps,
    option_pair: String,
    adjustment_factor: String,
) -> StdResult<()> {
    let attrs = vec![
        attr("action", "dynamic_pricing_adjusted"),
        attr("option_pair", option_pair.clone()),
        attr("adjustment_factor", adjustment_factor.clone()),
    ];
    Self::log_event(
        deps,
        RiskManagementEvent::DynamicPricingAdjusted {
            option_pair,
            adjustment_factor,
        },
        "dynamic_pricing_adjusted",
        attrs,
    )
}

pub fn emit_position_closed(deps: Deps, option_pair: String, amount: String) -> StdResult<()> {
    let attrs = vec![
        attr("action", "position_closed"),
        attr("option_pair", option_pair.clone()),
        attr("amount", amount.clone()),
    ];
    Self::log_event(
        deps,
        RiskManagementEvent::PositionClosed { ption_pair: option_pair, amount },
        "position_closed",
        attrs,
    )
}

pub fn emit_volatility_multiplier_adjusted(deps: Deps, volatility_multiplier: String) -> StdResult<()> {
    let attrs = vec![
        attr("action", "volatility_multiplier_adjusted"),
        attr("volatility_multiplier", volatility_multiplier.clone()),
    ];
    Self::log_event(
        deps,
        RiskManagementEvent::VolatilityMultiplierAdjusted {
            volatility_multiplier,
        },
        "volatility_multiplier_adjusted",
        attrs,
    )
}

fn log_event(
    _deps: Deps,
    event: RiskManagementEvent,
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
    let custom_event_name = format!("RiskManagementEvent_{}", event_type);

    // Create a response and add the custom event to it
    let response: Response = cosmwasm_std::Response::new().add_event(custom_event);
    //let response: Response<T> = cosmwasm_std::Response::new().add_event(custom_event);

    // Add default "wasm" event with additional attributes if needed
    response.add_attribute("action", custom_event_name);

    // Return the response
    Ok(())
}
}








/*In this events.rs file for the risk_management contract, we define several event types based on the risk management functionality:

PositionLimitSet: Emitted when a position limit is set for a specific option pair. It includes the option_pair and the max_position limit.

CircuitBreakerSet: Emitted when a circuit breaker is set for a specific option pair. It includes the option_pair, price_threshold, and triggered status.

RiskMitigationStrategyExecuted: Emitted when the risk mitigation strategy is executed.

DynamicPricingAdjusted: Emitted when dynamic pricing adjustments are made for a specific option pair. It includes the option_pair and the adjustment_factor.

PositionClosed: Emitted when a position is closed for a specific option pair. It includes the option_pair and the amount of the closed position.

VolatilityMultiplierAdjusted: Emitted when the volatility multiplier is adjusted based on risk assessments. It includes the updated volatility_multiplier.

The corresponding emit functions (e.g., emit_position_limit_set, emit_circuit_breaker_set, etc.) are used to emit the respective events.
 They take the necessary parameters, 
create the appropriate attributes, and call the log_event function to serialize the event data, create a custom event with attributes, 
and return a response with the event.

The log_event function follows the same pattern as in the previous examples, serializing the event data, adding the serialized data 
as an attribute, creating a custom event with the given type and attributes, and returning a response with the event.
You can use these event emission functions in the relevant parts of the risk_management contract to emit events when position
 limits are set, circuit breakers are triggered, risk mitigation strategies are executed, dynamic pricing adjustments are made, 
 positions are closed, or the volatility multiplier is adjusted.
 */