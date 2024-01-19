use cosmwasm_std::{attr, Attribute, StdResult, Event, Addr, to_json_string}; 
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, Response};

//Events implementtion
#[cw_serde]
pub enum ConstellationDerivativeEvent {
    OptionCreated { id: u64 },
    OptionClaimed { id: u64 },
    WithdrawCollateral{sender: Addr, id: u64},
    OptionTransferred { id: u64, new_owner: String },
    OptionExecuted{id: u64},
    OptionBurned {id: u64,sender: String,/*creator: String,*/},
}


impl ConstellationDerivativeEvent {
    pub fn emit_execute_option(deps: Deps, id: u64) -> StdResult<()> {
        let attrs = vec![
            attr("action", "execute_option"),
            attr("id", id.to_string()),
        ];

        Self::log_event(deps, ConstellationDerivativeEvent::OptionExecuted { id }, "execute_option", attrs)
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


    fn log_event(_deps: Deps, event: ConstellationDerivativeEvent, event_type: &str, attrs: Vec<Attribute>) -> StdResult<()> {
        // Serialize the event data into a string
        //let event_data_str = serde_json::to_string(&event)?;

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