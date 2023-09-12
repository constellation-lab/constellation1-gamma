# constelllation1_options

The Constellation GammaProtocol aka constellation is a decentralized options protocol that allows users to create arbitrary option tokens. 
This project implements an options trading contract code for the nibiru chain. Full details, description including maps and datastructures, the running baseline app and a video demo link can be found at this link: https://dorahacks.io/buidl/7215#details . There is also basic documentation on the app usage at https://app.gitbook.com/o/clGAVi5OLWOza6FgnAQq/s/ByGkTs5XC9ktsDLMFs7U/ and instructions for the app demo at https://docs.google.com/document/d/1FNbR0X8JjkzxmR6RACcf20Hko78IqNgNdmEeCiAVlGw/edit?usp=sharing . 

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
In summary, our code implements core options trading functionality like creation, buying, selling, exercising with expiration on the nibiru blockchain. There are big plans for constellation and a lot of work to be done however the portion of interest focused on so far is https://github.com/constellation-lab/constellation1-gamma/tree/main/contracts/option folder only, while we work towards completing all the below in future:
### you can find the contract code in /contracts/option 
you can run ```cargo wasm``` at /contracts/option to build the wasm code
Root Folder
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
    - opyn_cosmwasm.yaml
    - opyn_cosmwasm.toml
  - vesting
    - vesting.md
  - docs
    - README.md
    - getting_started.md
    - api_reference.md
    - faq.md



