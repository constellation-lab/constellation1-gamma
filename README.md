# constelllation1_options

The Constellation GammaProtocol aka constellation is a decentralized options protocol that allows users to create arbitrary option listings. As the name implies constellation is meant to launch web3 derivatives across various blockchains and across various blockchain ecosystems in strive towards interoperability among them, hence a constellation of web3 derivative products that promote the adoption of web3 derivatives at all levels.
This project implements an options trading contract code in rust configured for the nibiru chain and intends to build a complete options trading platform. . 

We implemented an options trading smart contract for the nibiru blockchain. Our code allows creating new options, specifying a collateral amount, counter offer amount, expiration time, amongst others and we intend to develop complex strategies in future. However, at this point the options system we developed has the details below:
Options are created by specifying collateral, a counter offer amount, and expiration time. The creator stakes assets into the option upon creation.
The option is owned by the creator initially. It exists in a base state not listed on the market.
Options can be listed by the owner on a market for sale by setting a price. Others can then buy the options.
They do this (buy/purchase the listed options) by providing the listed price. This transfers ownership. Purchasing removes the option from the market automatically.
Other aspects include: Owners can update the list price while on the market and Options can also be removed from the market without being purchased.
Options can be transferred to new owners. The code keeps track of option ownership and creation using maps/lists.

In addition, Options have an expiration time. After expiry, the creator can claim the collateral.
Before expiry, Options can be executed by the owner by providing the correct counter offer amount. This sends the counter offer tokens to creator and returns the staked collateral back to owner.
If the expiration time is reached, anyone (typically the creator) can claim the option. This returns the staked collateral to the creator and deletes the option.
The owner can also burn an unexpired option, which acts like a claim but is only callable by the owner.
Various queries are supported to get option details (by ID), get all options, get options by owner/creator address, paginated options, among others.
In summary, our code implements core options trading functionality like creation, buying, selling, exercising with expiration on the nibiru blockchain. 

you can run ```cargo wasm``` at /contracts/option to build the wasm code
Additional functionality in progress are as below:
Bidding, offering, accepting bids/offers, withdrawing collateral (including early withdrawal), extending expiration, pausing/unpausing the contract, and emitting events for transparency.

Other functionality being worked on/ added are time buffers before expiry:to prevent last minute sniping, Configurable buffer duration and Implementing partial withdrawals to Allow users to withdraw a % of their bid. Also, the use paginated returns for mappings where necessary and in addition the allowance of partial execution of an option by owner. The Addition an oracle for the latest price into the contract for market options. Allowing a fraction of an option to be traded, allowing various IBC tokens to be used as collateral, Allowing the owner to withdraw collateral early if not executed and Validating addresses where necessary.



  - README.md
  - LICENSE
  - CHANGELOG.md
  - scripts
    - build.sh
    - test.sh
  - contracts
    - calliope.rs
    - keeper.rs
    - lib.rs
    - option
    - price_oracle.rs
    - strike_oracle.rs
  - test
    - integration_test.rs
    - unit_test.rs
  - deployment
    - constella_cosmwasm.yaml
    - constella_cosmwasm.toml
  - vesting
    - vesting.md
  - docs
    - README.md
    - getting_started.md
    - api_reference.md
    - faq.md



