Opyn Cosm Wasm API Reference
This document provides a reference for the Opyn Cosm Wasm API. The Opyn Cosm Wasm API is a set of functions that can be used to interact with the Opyn Cosm Wasm contract.

Functions
The Opyn Cosm Wasm API provides the following functions:

create_option(): Creates a new option.
exercise_option(): Exercises an option.
get_option_info(): Gets information about an option.
list_options(): Lists all options.
cancel_order(): Cancels an order.
get_order_info(): Gets information about an order.
list_orders(): Lists all orders.
Parameters
The Opyn Cosm Wasm API functions take the following parameters:

sender: The address of the sender.
recipient: The address of the recipient.
underlying_asset: The underlying asset.
strike_price: The strike price.
expiration_date: The expiration date.
amount: The amount of options to create or exercise.
order_id: The order ID.
Returns
The Opyn Cosm Wasm API functions return the following values:

option_id: The ID of the newly created option.
bool: True if the option was successfully exercised, False otherwise.
OptionInfo: Information about the option.
OrderInfo: Information about the order.
[]OrderInfo: A list of orders.
Errors
The Opyn Cosm Wasm API functions may return the following errors:

INVALID_ARGUMENT: One or more of the arguments are invalid.
INSUFFICIENT_BALANCE: The sender does not have enough funds to create or exercise the option.
EXPIRED_OPTION: The option has expired.
INVALID_OPTION_ID: The option ID is invalid.
INVALID_ORDER_ID: The order ID is invalid.
Example
The following example shows how to create a new option:

Code snippet
import { OpynCosmWasm } from '@opyn/opyn-cosm-wasm';

const opynCosmWasm = new OpynCosmWasm();

const optionInfo = await opynCosmWasm.createOption({
  sender: 'cosmos1q234567890abcdefghijklmnopqrstuv',
  recipient: 'cosmos1uvwxyz01234567890abcdefghi',
  underlying_asset: 'uatom',
  strike_price: 100,
  expiration_date: '2023-05-15T00:00:00Z',
  amount: 1,
});

console.log(optionInfo);
Use code with caution. Learn more
This example will create a new option with the following parameters:

Sender: cosmos1q234567890abcdefghijklmnopqrstuv
Recipient: cosmos1uvwxyz01234567890abcdefghi
Underlying asset: uatom
Strike price: 100
Expiration date: 2023-05-15T00:00:00Z
Amount: 1
The output of the example will be the following:

Code snippet
{
  "option_id": "1",
  "underlying_asset": "uatom",
  "strike_price": 100,
  "expiration_date": "2023-05-15T00:00:00Z",
  "amount": 1,
  "sender": "cosmos1q234567890abcdefghijklmnopqrstuv",
  "recipient": "cosmos1uvwxyz01234567890abcdefghi",
  "created_at": "2023-05-14T23:59:59.999Z",
  "updated_at": "2023-05-14T23:59:59.999Z"
}
