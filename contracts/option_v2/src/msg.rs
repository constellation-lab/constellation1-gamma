#[allow(unused_imports)]
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use cosmwasm_std::{Coin, Addr, Decimal};
use crate::state::{State,Data};
use std::collections::HashMap;

//use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    //Create a new option
    Create {counter_offer: Vec<Coin>, time_stamp: u64},
    Transfer {id: u64,to: String },
    // Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {id: u64},
    // Burn will release collateral if expired
    Burn {id: u64},
    //Claim expier options collectal ayback the to the creator
    ClaimCollateral{id: u64},
    //Split option to small option
    //e.g. percentage 40 means this option's collateral/countoffer will be 60% and will init a new option with other 40%
    Split{id:u64, percentage:u64},
    // approve a address control your option
    Approve{spender:String},
    // disapprove a address control your opiton
    DisApprove{spender: String},

}

   
    
   

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(OptionsResponse)]
    Options{},
    #[returns(OptionsResponse)]
    OptionsPage{key: u64,amount: u64},
    #[returns(GetOptionByIdResponse)]
    GetOptionByid{id:u64},
    #[returns(OptionsResponse)]
    CreateorOptions{addr: String},
    #[returns(OptionsResponse)]
    OwnerOptions{addr: String},
    #[returns(bool)]
    GetIsApprove{spender:String,owner:String}

}

pub type ConfigResponse = State;
pub type OptionsResponse = Vec<(u64,Data)>;
pub type GetOptionByIdResponse = Data;




















/* Future msg Catalogue Code Aid for implementing Placeholders


//CW20 Interactions - To allow CW20 token locking, minting, burning etc:
pub enum Cw20ExecuteMsg {
  Transfer {
    recipient: String,
    amount: Uint128,    
  },

  Burn {
    amount: Uint128    
  } 
}

pub struct Cw20ReceiveMsg {
  sender: String,
  amount: Uint128,
  msg: Binary  
}


//Governance Integration
//To dispatch governance proposals:
pub enum GovExecuteMsg {
  Vote {
    proposal_id: u64,
    vote: VoteOption
  }  
}

pub enum VoteOption {
  Yes,
  No,
  Abstain
}

//Staking/Farming
//To wrap tokens and provide liquidity:
pub enum StakingMsg {
  Bond {
    amounts: Vec<Coin>
  },
  
  Unbond {
    amounts: Vec<Coin>  
  },
  
  Claim {}
}


//AMMs and DEXs
//Messages to enable trading on AMMs and DEXs:

pub enum DexMsg {
  Swap {
    route: Route,
    recipient: Option<String>
  },

  ProvideLiquidity {
    assets: [Coin; 2],
    slippage_tolerance: Option<Decimal>  
  },
   
  WithdrawLiquidity {
    amount: Uint128  
  }  
}

pub struct Route {
  source: AssetInfo,
  target: AssetInfo
}
  
pub struct AssetInfo {
  token: Addr,
  amount: Uint128  
}


//Oracle Interactions
//To query price feeds and data aggregators:
pub enum OracleMsg {

  GetPrice {
    base: String,
    quote: String
  },
  
  GetRate {
    from: String, 
    to: String
  }  
}


//Identity and Access
//For authentication/authorization:
pub enum IdentityMsg {

  SetClaims {
    claims: Claims  
  },
   
  CheckPermissions {
    action: String,
    resource: String
  }  
} 

pub struct Claims {
  permissions: HashMap<String, Vec<String>> 
}

//Atomic Swaps
//For trustless P2P token trading:
pub enum SwapMsg {

  CreateSwap {
    amounts: [Coin; 2],
    expiration: u64    
  },
  
  ClaimSwap { 
    swap_id: u64
  },
   
  RefundSwap {
    swap_id: u64  
  }
  
}

//Options V2
//Improved option contract interactions:
pub enum OptionV2Msg {

  CreateOption {
    amount: Uint128,
    strike_price: Decimal, 
    expiration: u64
  },
   
  ExerciseOption {
    option_id: u64
  },
   
  SettleOptions {
    start_id: u64,
    end_id: u64  
  }  
}
//NFT Options
//Message variants for NFT options
pub enum NftOptionMsg {

  CreateNftOption {
    nft_id: String,
    strike_price: Uint128,
    expiration: u64
  },

  ExerciseNftOption {
    option_id: u64  
  },

  SettleNftOption {
    option_id: u64  
  }

}

//NFT Staking
//To allow staking NFTs to earn yield:
pub enum NftStakingMsg {

  StakeNft {
    nft_id: String,
    duration: u64
  },

  UnstakeNft {
    nft_id: String
  },

  ClaimNftReward {
    nft_id: String  
  }
  
}
//For NFT backed loans:
pub enum NftLendingMsg {

  BorrowNft {
    nft_id: String,
    amount: Uint128,
    duration: u64    
  },

  RepayNftLoan {
    loan_id: u64  
  }

}

//Synthetic Assets
//Create "synthetic" assets that track real-world values:
pub enum SynthMsg {

  MintToken {
    synth_type: String, 
    collateral_amount: Uint128
  }

  BurnToken {
    amount: Uint128    
  }

}

pub enum SynthType {
  sUSD, 
  sEUR,
  sJPY // fiat currency proxies  

  Gold, 
  Silver, 
  Oil // commodities

  TSLA,
  AAPL // equities
}


//Basket Options
//Options with an underlying basket of assets:

pub struct BasketOption {
  assets: Vec<Asset>,
  amounts: Vec<Uint128>,  
  strike_price: Decimal,
  expiration: u64
}

pub enum BasketAsset {
  Token { contract_addr: Addr },
  Synth { synth_type: SynthType }  
}

//Dynamic NFTs
//NFTs with mutable state

pub enum DynamicNftMsg {

  SetStats {
    token_id: String,
    stats: BTreeMap<String, String>
  }

  SetLevel {
    token_id: String,
    level: u32
  }  

}

//Reputation System
//On-chain credibility tracking:
pub enum RepMsg {

  AddCred {
    user: String,
    cred: u8 
  },

  RemoveCred {  
    user: String,
    cred: u8
  }  

  GetCredScore { user: String }  

}


//DAO Integrations
//Integrate with decentralized autonomous organizations:

pub enum DaoMsg {

  SubmitProposal {
    title: String,
    description: String,
    action: ProposalAction  
  },

  Vote {
    proposal_id: u64, 
    vote: VoteOption
  },

  ExecuteProposal {
    proposal_id: u64  
  },

  WithdrawFunds {
    recipient: String,
    amount: Uint128    
  }

}

pub enum ProposalAction {
  CallContract {
    contract_addr: Addr,
    msg: Binary  
  },
  TransferAssets {
    recipient: String,
    amounts: Vec<Coin>
  }
}

//Advanced Market Mechanisms
//Like prediction markets, futures, etc:

pub struct PredictionMarket {
  question: String,
  end_time: u64,
  // Outcomes
}

pub enum PredictionMsg {

  CreateMarket {
    market: PredictionMarket    
  },

  PlacePrediction {
    market_id: u64,
    outcome: u8,
    amount: Uint128
  }  

}

//Inter-Blockchain Communication
//Connect with other chains via IBC:

pub enum IbcMsg {
  
  SendPacket {
    channel_id: String,
    data: Binary,
    timeout_height: u64, 
    timeout_timestamp: u64
  }

  RecvPacket {
    packet_id: u64  
  }
  
}

//Decentralized Social Graph
//User-owned social connections:

pub struct Relationship {
  related_account: String,
  type: RelationshipType,
  metadata: Option<Binary>  
}

pub enum RelationshipType {
  Friend,
  Colleague, 
  Family 
}

pub enum SocialGraphMsg {

  CreateRelationship {
    relationships: Vec<Relationship>
  }

  UpdateRelationship {
    counterparty: String,
    relationship: Relationship  
  }

  RemoveRelationship { 
    counterparty: String
  }

  FindRelatedUsers {
    user: String,
    relationship_types: Vec<RelationshipType>  
  }

}


//Decentralized Derivatives Protocol
//On-chain asset backed derivative creation:
pub struct Derivative {  
  underlying_asset: Asset,
  asset_ratio: Decimal,  
  strike_price: Decimal,
  expiration: u64,
  is_put: bool
}

pub enum Asset {
  NativeToken { denom: String },
  Token { contract: Addr }    
}

pub enum DerivativeMsg {

  CreateDerivative {
    derivative: Derivative
  }

  MintToken {
    derivative_id: u64,
    collateral_amount: Uint128  
  }
   
  BurnToken {
    amount: Uint128    
  }

  SettlePayout {
    derivative_id: u64
  }

}

//Decentralized Exchange Protocol
//On-chain automated market maker and orderbook dex:
pub enum DexMsg {

  AddLiquidity {
    asset_infos: [AssetInfo; 2],
    slippage_tolerance: Decimal
  }

  RemoveLiquidity {
    lp_token_amount: Uint128    
  }

  Swap {
    route: SwapRoute,
    minimum_receive: Uint128
  }

  PlaceAskOrder {
    trading_pair: TradingPair, 
    price: Decimal,
    amount: Uint128
  }

  PlaceBidOrder {
    trading_pair: TradingPair,
    price: Decimal, 
    amount: Uint128
  }

  CancelOrder { order_id: U64 }

}

pub struct AssetInfo {
  token: Addr,
  amount: Uint128  
}

pub struct SwapRoute {
  path: Vec<Addr>,
  amount: Uint128
}

pub struct TradingPair {
  asset_infos: [AssetInfo; 2]  
}


//Decentralized Autonomous Taxation
//Programmable tax policy and allocation:
pub enum TaxMsg {

  SetTaxPolicy {
    schema_version: String,
    policy: TaxPolicy    
  }

  CalculateOwed {
    entity: String,
    timestamp: u64
  }
   
  PayTaxes {
    // Payment info  
  }

  AllocateTaxes {
    group: String,
    amount: Uint128  
  }  
}

pub struct TaxPolicy {
  rates: BTreeMap<TaxType, Decimal>  
}

pub enum TaxType {  
  Income,
  CapitalGains,
  Sales  
}

//Decentralized Advertising Protocol
//Transparent and efficient on-chain advertising:
pub struct Advertisement {
  owner: Addr,
  targets: Vec<TargetingAttributes>, 
  bid: Uint128, 
  url: String,
  expiration: u64   
}

pub struct TargetingAttributes {
  demographics: BTreeMap<String, String>  
}

pub enum AdvertisingMsg {

  PublishAdvertisement {
    advertisement: Advertisement
  }

  ViewAdvertisement { owner: Addr }

  ClickAdvertisement { 
    advertisement_id: u64 
  }

}

 
//Decentralized Lending - Algorithmic interest rate model and undercollateralized loans

pub struct LoanTerms {
  amount: Uint128,
  duration: u64,
  apr: Decimal   
}

pub enum LendingMsg {

  OpenLoan {
    collateral: Vec<Coin>,
    terms: LoanTerms
  }

  MakePayment {
    loan_id: u64,  
    payment: Coin  
  }

  CloseLoan { loan_id: u64 } 

}



//Decentralized Insurance - Democratized and pooled risk coverage

pub enum ClaimType {
  Theft,
  PropertyDamage,
  Accident,
  Health
}

pub struct InsuranceClaim {
  claim_type: ClaimType,
  description: String, 
  evidence: Vec<u8> // IPFS hash  
}

pub enum InsuranceMsg {

  RequestCoverage {
    asset: Asset,
    amount: Uint128,
    duration: u64
  }

  MakeClaim {
    policy_id: u64,
    claim: InsuranceClaim
  }

  VoteOnClaim {
    claim_id: u64,
    vote: bool  
  }

  FinalizeClaim { claim_id: u64 }
  
}

pub enum Asset {
  Property {
    // Details    
  },
  Health {
    // Details
  }  
}


//Decentralized Machine Learning - Distributed model training with encryption (federated learning) 

pub struct ModelDefinition {
  model_type: String, // "Logistic Regression", "Random Forest"... 
  feature_space: Vec<String> // input features  
}  

pub struct TrainingData {
  observations: Vec<Observation>  
}

pub struct Observation {
  features: Vec<f32>,
  label: String // classification 
}

pub enum MlMsg {

  RegisterModel {
    model: ModelDefinition    
  }

  ContributeTrainingData {
    model_id: String,
    data: TrainingData
  }
  
  StartTrainingRound {
    model_id: String,  
    optimizer: String, // sgd, adam..
    config: TrainingConfig    
  }

}


//Decentralized Investment - Invest in baskets of digital assets algorithmically constructed based on risk model

pub struct AssetBundle {
  assets: Vec<InvestableAsset>,
  model_params: RiskModelParams  
}

pub enum InvestableAsset {
  Token { contract_addr: Addr }, 
  SynthAsset { type: String }      
}

pub struct RiskModelParams {
  risk_function: String, 
  constraints: Vec<String>       
}

pub enum InvestmentMsg {

  CreateBundleProposal {
    bundle: AssetBundle
  }

  SubmitBundleVote {
    proposal_id: u64,
    vote: bool
  }
   
  ExecuteBundleProposal {
    proposal_id: u64    
  }
  
}

//Decentralized Social Payments - Embed token rewards in content to monetize social media

pub struct SocialPost {
  content: String, 
  distribute_to_groups: Vec<String>   
}

pub struct RewardTerms {
  amount: Uint128,
  unlock_criteria: Vec<UnlockCriterion>  
}

pub enum UnlockCriterion {
  NumViews { min_views: u64 },
  TimeLocked { duration_secs: u64 }
}

pub enum SocialPaymentMsg {

  PublishPost {
    post: SocialPost,
    rewards: RewardTerms    
  }

  ViewPost { post_id: u64 }

}

//Decentralized Advertising Exchange - Match information consumers with providers through relevance-based bidding

pub struct Ad {
  owner: String,
  targets: Vec<AdTarget>,
  bid: Uint128,
  url: String,
  expiration: u64
}

pub struct AdTarget {
  topic: String, 
  target_score: u8 // relevance 
}  

pub enum AdExchangeMsg {

  SubmitAd {
    ad: Ad
  }

  ViewAd {
    user_topics: Vec<String> // based on profile    
  }

}


//Decentralized Prediction Information Marketplace - Monetize insider information to set probabilistic event outcomes

pub struct Prediction {
  question: String,
  outcomes: Vec<String>,
  end_time: u64  
}

pub enum PredictionMsg {

  SubmitPrediction {
    market_id: u64,
    outcome: u8, // enum index
    amount: Uint128
  }

  DisputePrediction {
    prediction_id: u64    
  }

  ResolvePrediction {
    market_id: u64     
  }

}
*/