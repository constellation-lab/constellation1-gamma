use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Binary};
use cw20::Logo;
use cw20::Expiration;


#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
}



#[cw_serde]
pub enum ExecuteMsg {
    Transfer { recipient: String, amount: Uint128 },
    Burn { amount: Uint128 },
    Mint { recipient: String, amount: Uint128 },
    Send { contract: String, amount: Uint128, msg: Binary },
    IncreaseAllowance { spender: String, amount: Uint128 },
    DecreaseAllowance { spender: String, amount: Uint128 },
    TransferFrom { owner: String, recipient: String, amount: Uint128 },
    BurnFrom { owner: String, amount: Uint128 },
    SendFrom { owner: String, contract: String, amount: Uint128, msg: Binary },
    Vote { proposal_id: u64, vote: bool },
}


#[cw_serde]
pub enum QueryMsg {
    Balance { address: String },
    TokenInfo {},
    Allowance { owner: String, spender: String },
    AllAllowances { owner: String, start_after: Option<String>, limit: Option<u32> },
    AllAccounts { start_after: Option<String>, limit: Option<u32> },
    Minter {},
    MarketingInfo {},
    DownloadLogo {},
    VotingPower { address: String },
}

#[cw_serde]
pub struct Cw20Coin {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

#[cw_serde]
pub struct StakeInfo {
    pub staked_amount: Uint128,
}