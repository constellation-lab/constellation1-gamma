use cosmwasm_std::{attr, Attribute, StdResult, Event, Addr, Decimal}; 
use std::collections::HashMap;
use cosmwasm_schema::{serde::{Serialize, Deserialize}};
use cosmwasm_std::{Deps, Response, Coin};

//Events implementtion
#[derive(Serialize, Deserialize, Debug)]
pub enum ConstellationDerivativeEvent {
    BidPlaced { bid_id: u64 },
    BidOrOfferAccepted { id: u64 },
    OptionCreated { id: u64 },
    OptionClaimed { id: u64 },
    OfferPlaced { id: u64 },
    PartialExecution{id: u64},
    FractionalBuy{id: u64, amount: String},
    WithdrawCollateral{sender: Addr, id: u64},
    ExtendExpiration{id: u64, new_expiration: u64},
    ContractPaused{},
    ContractUnpaused{},
    OracleAdded{sender: Addr, oracle: Addr},
    PriceUpdatedWithOracle{price: Vec<Coin>},
    OptionExerciseConditionsSet{id: u64, exercise_conditions: Vec<String>},
    OptionExpiredNotification { id: u64, notification_period: u64 },
    OptionParametersSet { id: u64, parameters: HashMap<String, String> },
    OptionHistory { id: u64, history: Vec<String> },
    OptionRiskMetricsCalculated { id: u64, risk_metrics: HashMap<String, Decimal> },
    LiquidityProvided { provider: String, amount: String },
    LiquidityWithdrawn { provider: String, amount: String },
    VoteOnGovernanceProposal { proposal_id: u64, vote: bool },
    OptionUsedAsCollateral { option_id: u64, amount: Vec<Coin> },
    OptionWrappedForYieldFarming { option_id: u64, amount: Vec<Coin> },
    AmmPoolCreated { option_id: u64 },
    OptionTradedOnAmm { pool_id: u64, amount: Vec<Coin> },
    MarketDataFeedIntegrated { option_id: u64, data_feed_url: String },
    TokensLocked { amount: Vec<Coin>, lock_duration: u64 },
    UserReferred { referred_user: Addr },
    DiscountCriteriaSet { user: Addr, criteria: HashMap<String, String> },
    OptionTransferred { id: u64, new_owner: String },
    OptionExecuted{id: u64},
    OptionExpired { id: u64 },
    PriceUpdated {id: u64, /*price: String*/},
    BuyOption{id: u64, /*price: String*/}, 
    OptionAddedToMarket {id: u64,/*amount: String,denom: String,*/},
    OptionBurned {id: u64,sender: String,/*creator: String,*/},
    OptionRemovedFromMarket {id: u64, /*sender: String,*/},
       // BidWithdrawn { bid_id: u64 },
}


impl ConstellationDerivativeEvent {
   
    pub fn emit_option_bought(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_bought"),
            attr("id", id.to_string()),
            //attr("price", price),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BuyOption { id, }, "option_bought", attrs)
    }

    pub fn emit_option_price_updated(deps: Deps, id: u64, /*price: Vec<Coin>*/) -> StdResult<()> {
        let attrs = vec![
            attr("action", "price_updated"),
            attr("id", id.to_string()),
            //attr("id", price[]),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PriceUpdated { id, /*price*/ }, "price_updated", attrs)
    }

    pub fn emit_execute_option(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "execute_option"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExecuted { id }, "execute_option", attrs)
    }

    pub fn emit_fractional_buy(deps: Deps, id: u64, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "fractional_buy"),
            attr("id", id.to_string()),
            attr("amount", amount.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::FractionalBuy { id, amount }, "fractional_buy", attrs)
    }

    pub fn emit_partial_execution(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "partial_execution"),
            attr("id", id.to_string()),
            
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PartialExecution { id }, "partial_execution", attrs)
    }

    pub fn emit_bid_placed(deps: Deps, bid_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_placed"),
            attr("bid_id", bid_id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BidPlaced { bid_id }, "bid_placed", attrs)
    }

    pub fn emit_offer_placed(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "offer_placed"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OfferPlaced { id }, "offer_placed", attrs)
    }

    pub fn emit_bid_or_offer_accepted(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_accepted"),
            attr("id", id.to_string()),
            //attr("option_id", option_id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::BidOrOfferAccepted { id }, "bid_or_offer_accepted", attrs)
    }

    pub fn emit_withdraw_collateral(deps: Deps, sender: Addr, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "withdraw_collateral"),
            attr("sender", sender.to_string()),
            attr("id", id.to_string()),
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::WithdrawCollateral { sender, id }, "withdraw_collateral", attrs)
    }

    pub fn emit_expiration_extended(deps: Deps, id: u64, new_expiration: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "expiration_extended"),
            attr("id", id.to_string()),
            attr("new_expiration", new_expiration.to_string()),
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::ExtendExpiration{ id, new_expiration }, "expiration_extended", attrs)
    }

        // Event for pausing the contract
    pub fn emit_contract_paused(deps: Deps) -> StdResult<()> {
        let attrs = vec![
            attr("action", "contract_paused"),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::ContractPaused {}, "contract_paused", attrs)
    }

    // Event for unpausing the contract
    pub fn emit_contract_unpaused(deps: Deps) -> StdResult<()> {
        let attrs = vec![
            attr("action", "contract_unpaused"),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::ContractUnpaused {}, "contract_unpaused", attrs)
    }

        // Event for adding an oracle
    pub fn emit_oracle_added(deps: Deps, sender: Addr, oracle: Addr) -> StdResult<()> {
        let attrs = vec![
            attr("action", "oracle_added"),
            attr("sender", sender.to_string()),
            attr("oracle", oracle.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OracleAdded { sender, oracle }, "oracle_added", attrs)
    }

    // Event for updating price with oracle
    pub fn emit_price_updated_with_oracle(deps: Deps, price: Vec<Coin>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "price_updated_with_oracle"),
            attr("price", format!("{:?}", price)),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::PriceUpdatedWithOracle { price }, "price_updated_with_oracle", attrs)
    }

    // Event for setting option exercise conditions
    pub fn emit_option_exercise_conditions_set(deps: Deps, id: u64, exercise_conditions: Vec<String>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_exercise_conditions_set"),
            attr("id", id.to_string()),
            attr("exercise_conditions", format!("{:?}", exercise_conditions)),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExerciseConditionsSet { id, exercise_conditions }, "option_exercise_conditions_set", attrs)
    }

    pub fn emit_option_expired_notification(
        deps: Deps,
        id: u64,
        notification_period: u64,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_expired_notification"),
            attr("id", id.to_string()),
            attr("notification_period", notification_period.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionExpiredNotification {
                id,
                notification_period,
            }, "expired_notification",
            attrs,
        )
    }

    pub fn emit_option_parameters_set(
        deps: Deps,
        id: u64,
        parameters: HashMap<String, String>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_parameters_set"),
            attr("id", id.to_string()),
            attr("parameters", format!("{:?}", parameters)), // Format HashMap as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionParametersSet { id, parameters }, "option_parameters_set",
            attrs,
        )
    }

    pub fn emit_liquidity_provided(deps: Deps, provider: String, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "liquidity_provided"),
            attr("provider", provider.clone()),
            attr("amount", amount.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::LiquidityProvided { provider, amount }, "liquidity_provided", attrs)
    }

    pub fn emit_liquidity_withdrawn(deps: Deps, provider: String, amount: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "liquidity_withdrawn"),
            attr("provider", provider.clone()),
            attr("amount", amount.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::LiquidityWithdrawn { provider, amount }, "liquidity_withdrawn" , attrs)
    }
    
    pub fn emit_option_history(deps: Deps, id: u64, history: Vec<String>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_history"),
            attr("id", id.to_string()),
            attr("history", history.join(", ")), // Join history into a single string for clarity
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::OptionHistory { id, history }, "option_history", attrs)
    }
    
    pub fn emit_option_risk_metrics_calculated(deps: Deps, id: u64, risk_metrics: HashMap<String, Decimal>) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_risk_metrics_calculated"),
            attr("id", id.to_string()),
            attr("risk_metrics", format!("{:?}", risk_metrics)), // Format HashMap as a string for clarity
        ];
    
        Self::log_event(deps, ConstellationDerivativeEvent::OptionRiskMetricsCalculated { id, risk_metrics }, "option_risk_metrics_calculated", attrs)
    }
    

    pub fn emit_vote_on_governance_proposal(
        deps: Deps,
        proposal_id: u64,
        vote: bool,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "vote_on_governance_proposal"),
            attr("proposal_id", proposal_id.to_string()),
            attr("vote", vote.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::VoteOnGovernanceProposal { proposal_id, vote }, "vote_on_governance_proposal",
            attrs,
        )    
    }

    pub fn emit_option_used_as_collateral(
        deps: Deps,
        option_id: u64,
        amount: Vec<Coin>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_used_as_collateral"),
            attr("option_id", option_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionUsedAsCollateral { option_id, amount }, "option_used_as_collateral", 
            attrs,
        )
    }
    
    pub fn emit_option_wrapped_for_yield_farming(
        deps: Deps,
        option_id: u64,
        amount: Vec<Coin>,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_wrapped_for_yield_farming"),
            attr("option_id", option_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::OptionWrappedForYieldFarming { option_id, amount }, "option_wrapped_for_yield_farming",
            attrs,
        )
    }

    pub fn emit_amm_pool_created(deps: Deps, option_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "amm_pool_created"),
            attr("option_id", option_id.to_string()),
        ];
    
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::AmmPoolCreated { option_id }, "amm_pool_created",
            attrs,
        )
    }
    
    pub fn emit_option_traded_on_amm(deps: Deps, pool_id: u64, amount: Vec<Coin>,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_traded_on_amm"),
            attr("pool_id", pool_id.to_string()),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::OptionTradedOnAmm { pool_id, amount }, "option_traded_on_amm", attrs,
        )
    }
    
    pub fn emit_market_data_feed_integrated(deps: Deps, option_id: u64, data_feed_url: String,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "market_data_feed_integrated"),
            attr("option_id", option_id.to_string()),
            attr("data_feed_url", data_feed_url.clone()),
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::MarketDataFeedIntegrated { option_id, data_feed_url }, "market_data_feed_integrated", attrs,
        )
    }

    
    pub fn emit_user_referred(deps: Deps, referred_user: Addr) -> StdResult<()> {
        let attrs = vec![
            attr("action", "user_referred"),
            attr("referred_user", referred_user.clone()),
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::UserReferred { referred_user }, "user_referred", attrs,
        )
    }
    
    pub fn emit_discount_criteria_set(deps: Deps, user: Addr, criteria: HashMap<String, String>,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "discount_criteria_set"),
            attr("user", user.clone()),
            attr("criteria", format!("{:?}", criteria)), // Format HashMap as a string for clarity
        ];
        Self::log_event(
            deps, ConstellationDerivativeEvent::DiscountCriteriaSet { user, criteria }, "discount_criteria_set", attrs,
        )
    }

    pub fn emit_option_created(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_created"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionCreated { id }, "option_created",attrs)
    }

    pub fn emit_option_claimed(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_claimed"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionClaimed { id }, "option_claimed", attrs)
    }

    pub fn emit_option_transferred(deps: Deps, id: u64, new_owner: String) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_transferred"),
            attr("id", id.to_string()),
            attr("new_owner", new_owner.clone()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionTransferred { id, new_owner }, "option_transferred", attrs)
    }

    pub fn emit_option_expired(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_expired"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExpired { id }, "option_expired", attrs)
    }

    pub fn emit_option_added_to_market(
        deps: Deps,
        id: u64,
        //amount: String,
        //denom: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_added_to_market"),
            attr("id", id.to_string()),
            //attr("amount", amount.clone()),
            //attr("denom", denom.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionAddedToMarket { id, /*amount, denom*/ }, "option_added_to_market",
            attrs,
        )
    }

    pub fn emit_option_burned(
        deps: Deps,
        id: u64,
        sender: String,
        //creator: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_burned"),
            attr("id", id.to_string()),
            attr("sender", sender.clone()),
            //attr("creator", creator.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionBurned {
                id,
                sender,
              //  creator,
            }, "option_burned",
            attrs,
        )
    }

    pub fn emit_option_removed_from_market(
        deps: Deps,
        id: u64,
        //sender: String,
    ) -> StdResult<()> {
        let attrs = vec![
            attr("action", "option_removed_from_market"),
            attr("id", id.to_string()),
            //attr("sender", sender.clone()),
        ];

        Self::log_event(
            deps.into(),
            ConstellationDerivativeEvent::OptionRemovedFromMarket { id, /*sender*/ }, "option_removed_from_market",
            attrs,
        )
    }

   /* pub fn emit_bid_withdrawn(deps: Deps, bid_id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "bid_withdrawn"),
            attr("bid_id", bid_id.to_string()),
        ];
        Self::log_event(deps, ConstellationDerivativeEvent::BidWithdrawn { bid_id }, attrs)
    }

    pub fn emit_tokens_locked(
        deps: Deps, amount: Vec<Coin>, lock_duration: u64,) -> StdResult<()> {
        let attrs = vec![
            attr("action", "tokens_locked"),
            attr("amount", format!("{:?}", amount)), // Format Vec<Coin> as a string for clarity
            attr("lock_duration", lock_duration.to_string()),
        ];
        Self::log_event(
            deps,
            ConstellationDerivativeEvent::TokensLocked { amount, lock_duration }, "tokens_locked" ,
            attrs,
        )
    }
*/

    fn log_event(_deps: Deps, event: ConstellationDerivativeEvent, event_type: &str, attrs: Vec<Attribute>) -> StdResult<()> {
        // Serialize the event data into a string
        //let event_data_str = serde_json::to_string(&event)?;

        let event_data_str = match serde_json::to_string(&event) {
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
        let custom_event_name = format!("ConstellationDerivativeEvent_{}", event_type);
        // Create a response and add the custom event to it
        let response: Response<()> = Response::new().add_event(custom_event);
        // Add default "wasm" event with additional attributes if needed
        response.add_attribute("action", custom_event_name);
        // Return the response
        //Ok(response)
        Ok(())
    }
}