use {
    anchor_lang::prelude::*,
    bytemuck::{Pod, Zeroable},
};

pub const MAX_RESOLVED_ROUNDS: usize = 32;

pub const MAX_CUSTODIES: usize = 10;

pub const MAX_STABLE_CUSTODY: usize = 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct ClosePositionLongParams {
    pub price: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct ClosePositionShortParams {
    pub price: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LiquidateLongParams {}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LiquidateShortParams {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, Pod, Zeroable)]
#[repr(C)]
pub struct LimitedString {
    pub value: [u8; 31],
    pub length: u8,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct TradingStats {
    pub opened_position_count: u64,
    pub liquidated_position_count: u64,
    // At position opening, in bps
    pub opening_average_leverage: u64,
    pub opening_size_usd: u64,
    // Calculated when position close/get liquidated
    pub profits_usd: u64,
    pub losses_usd: u64,
    pub fee_paid_usd: u64,
}

#[account(zero_copy)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct UserProfile {
    pub bump: u8,
    pub _padding: [u8; 7],
    pub nickname: LimitedString,
    pub created_at: i64,
    //
    pub owner: Pubkey,
    //
    pub swap_count: u64,
    pub swap_volume_usd: u64,
    pub swap_fee_paid_usd: u64,
    //
    pub short_stats: TradingStats,
    pub long_stats: TradingStats,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct StakingRound {
    pub start_time: i64,
    // Unknown until ended (min value being ROUND_MIN_DURATION_SECONDS from start_time)
    pub end_time: i64,
    //
    // The amount of reward you get per staked stake-token for that round - set at Round's resolution
    pub rate: u64,
    // Set at Round's resolution
    pub total_stake: u64,
    // Set at Round's resolution
    pub total_claim: u64,
    //
    // The amount of lm reward you get per staked stake-token for that round - set at Round's resolution
    pub lm_rate: u64,
    // Set at Round's resolution
    pub lm_total_stake: u64,
    // Set at Round's resolution
    pub lm_total_claim: u64,
}

#[account(zero_copy)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct Staking {
    pub staking_type: u8, // StakingType
    //
    // Bumps
    //
    pub bump: u8,
    pub staked_token_vault_bump: u8,
    pub reward_token_vault_bump: u8,
    pub lm_reward_token_vault_bump: u8,
    //
    pub reward_token_decimals: u8,
    pub staked_token_decimals: u8,
    //
    pub initialized: u8, // StakingInitializationStep
    //
    // Tokens in stake
    //
    pub nb_locked_tokens: u64,
    pub nb_liquid_tokens: u64,
    //
    // Token to stake
    //
    pub staked_token_mint: Pubkey,
    //
    // Resolved amounts
    //
    // amount of rewards allocated to resolved rounds, claimable (excluding current/next round)
    pub resolved_reward_token_amount: u64,
    // amount of staked token locked in resolved rounds
    pub resolved_staked_token_amount: u64,
    // amount of lm rewards allocated to resolved rounds, claimable (excluding current/next round)
    pub resolved_lm_reward_token_amount: u64,
    // amount of lm staked token locked in resolved rounds
    pub resolved_lm_staked_token_amount: u64,
    //
    // Staking rounds
    //
    pub current_staking_round: StakingRound,
    pub next_staking_round: StakingRound,
    pub resolved_staking_rounds: [StakingRound; MAX_RESOLVED_ROUNDS],
    pub registered_resolved_staking_round_count: u8,
    //
    pub _padding: [u8; 3],
    //
    // Token LM emission potentiometer
    // baseline is 10_000 (100%)
    pub lm_emission_potentiometer_bps: u16,
    pub months_elapsed_since_inception: u16,
    pub resolve_round_cron_thread_id: u64,
    // Date at with the `current_month_emission_amount_per_round` was calculated last
    pub emission_amount_per_round_last_update: i64,
    // Amount of rewards to be distributed per staking round
    pub current_month_emission_amount_per_round: u64,
}

#[account(zero_copy)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct Cortex {
    pub bump: u8,
    pub transfer_authority_bump: u8,
    pub lm_token_bump: u8,
    pub governance_token_bump: u8,
    pub initialized: u8, // CortexInitializationStep
    pub fee_conversion_decimals: u8,
    pub _padding: [u8; 2],
    pub lm_token_mint: Pubkey,
    //
    pub inception_time: i64,
    pub admin: Pubkey,
    //
    // Depending of the context:
    // - convert collected fees into this mint
    // - distribute rewards in this mint
    //
    pub fee_redistribution_mint: Pubkey,
    pub protocol_fee_recipient: Pubkey, // a wallet from the DAO that will handle buybacks
    //
    pub pools: [Pubkey; 4],
    pub user_profiles_count: u64,
    //
    // Governance
    //
    pub governance_program: Pubkey,
    pub governance_realm: Pubkey,
    //
    // LM Token buckets' allocations
    //
    pub core_contributor_bucket_allocation: u64,
    pub foundation_bucket_allocation: u64,
    pub ecosystem_bucket_allocation: u64,
    //
    // Buckets stats
    //
    pub core_contributor_bucket_vested_amount: u64,
    pub core_contributor_bucket_minted_amount: u64,
    pub foundation_bucket_vested_amount: u64,
    pub foundation_bucket_minted_amount: u64,
    pub ecosystem_bucket_vested_amount: u64,
    pub ecosystem_bucket_minted_amount: u64,
    //
    // genesis Lock campaign contributions
    pub genesis_liquidity_alp_amount: u64,
    // unique thread id counter for automation thread on positions (incremented with wrapping add, looping)
    pub unique_position_automation_thread_id_counter: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct TokenRatios {
    pub target: u16,
    pub min: u16,
    pub max: u16,
    pub _padding: [u8; 2],
}

// U128Split is a struct that represents a u128 as two u64s for zero_copy needs
#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct U128Split {
    high: u64,
    low: u64,
}

#[account(zero_copy)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct Pool {
    pub bump: u8,
    pub lp_token_bump: u8,
    pub nb_stable_custody: u8,
    //
    pub initialized: u8, // 0 = false, 1 = true
    //
    // Permissions (applicable to all custodies)
    //
    // If false, make positions readonly/closeonly
    pub allow_trade: u8,
    pub allow_swap: u8,
    //
    pub liquidity_state: u8, // PoolLiquidityState,
    //
    pub registered_custody_count: u8,
    //
    pub name: LimitedString,
    //
    pub custodies: [Pubkey; MAX_CUSTODIES],
    pub ratios: [TokenRatios; MAX_CUSTODIES],
    pub aum_usd: U128Split,
    //
    pub inception_time: i64,
    //
    // Ideal max AUM value for the pool
    // It's a soft limit considering the assets in the pool can increase of value,
    // thus making the AUM grow higher than the limit
    pub aum_soft_cap_usd: u64,
}

#[account(zero_copy)]
#[derive(Default, Debug)]
#[repr(C)]
pub struct Position {
    pub bump: u8,
    pub side: u8, // Side
    pub take_profit_thread_is_set: u8,
    pub stop_loss_thread_is_set: u8,
    pub pending_cleanup_and_close: u8,
    pub _padding: [u8; 3],

    pub owner: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,

    pub open_time: i64,
    pub update_time: i64,
    pub price: u64,
    pub size_usd: u64,
    pub borrow_size_usd: u64,
    pub collateral_usd: u64,
    // Borrowing fee to be paid (increasing after increase position)
    pub unrealized_interest_usd: u64,
    pub cumulative_interest_snapshot: U128Split,
    pub locked_amount: u64,
    pub collateral_amount: u64,

    // Estimations at open - When actually closing, the confidence will be applied
    // Before being used again at close time, these values are updated to their true values
    pub exit_fee_usd: u64,
    pub liquidation_fee_usd: u64,

    pub take_profit_thread_id: u64,
    pub take_profit_limit_price: u64,
    pub stop_loss_thread_id: u64,
    pub stop_loss_limit_price: u64,
    // 0 means no slippage
    pub stop_loss_close_position_price: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct PricingParams {
    // Pricing params have implied BPS_DECIMALS decimals (except ended with _usd)
    pub max_initial_leverage: u32,
    pub max_leverage: u32,
    // One position size can't exceed this amount
    pub max_position_locked_usd: u64,
    // Limit the total size of short positions for the custody
    pub max_cumulative_short_position_size_usd: u64,
}

// Fees have implied BPS_DECIMALS decimals
#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct Fees {
    // Base fees
    pub swap_in: u16,
    pub swap_out: u16,
    pub stable_swap_in: u16,
    pub stable_swap_out: u16,
    pub add_liquidity: u16,
    pub remove_liquidity: u16,
    pub close_position: u16,
    pub liquidation: u16,
    pub fee_max: u16,
    pub _padding: [u8; 6],
    pub _padding2: u64, // force u64 alignment
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct BorrowRateParams {
    // Borrow rate params have implied RATE_DECIMALS decimals
    pub max_hourly_borrow_interest_rate: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct FeesStats {
    pub swap_usd: u64,
    pub add_liquidity_usd: u64,
    pub remove_liquidity_usd: u64,
    pub close_position_usd: u64,
    pub liquidation_usd: u64,
    pub borrow_usd: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct VolumeStats {
    pub swap_usd: u64,
    pub add_liquidity_usd: u64,
    pub remove_liquidity_usd: u64,
    pub open_position_usd: u64,
    pub close_position_usd: u64,
    pub liquidation_usd: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct TradeStats {
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub oi_long_usd: u64,
    pub oi_short_usd: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct Assets {
    // Collateral debt
    pub collateral: u64,
    // Owned = total_assets - collateral + collected_fees - protocol_fees
    pub owned: u64,
    // Locked funds for pnl payoff
    pub locked: u64,
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct StableLockedAmountStat {
    pub custody: Pubkey,
    pub locked_amount: u64,
    pub _padding: [u8; 8],
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct PositionsAccounting {
    pub open_positions: u64,
    pub size_usd: u64,
    pub borrow_size_usd: u64,
    pub locked_amount: u64,
    pub weighted_price: U128Split,
    pub total_quantity: U128Split,
    pub cumulative_interest_usd: u64,
    pub _padding: [u8; 8],
    pub cumulative_interest_snapshot: U128Split,
    // This exit fee stats is used to calculate the PnL of all opened positions, it is not reflecting the actual exit fee of the position (that can sometimes be lesser)
    // so that the AUM take into account an approximation of the PnL of all opened positions
    pub exit_fee_usd: u64,
    //
    // Store the stable custody locked amount related to this custody
    //
    // Example:
    // When Shorting 1 ETH, 1500 USDC get locked to provide for trader maximum payoff
    // USDC custody locked amount: +1500
    // eth custody stable locked amount: +1500
    //
    // Needed to be able to calculate PnL
    pub stable_locked_amount: [StableLockedAmountStat; MAX_STABLE_CUSTODY],
}

#[derive(
    Copy, Clone, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug, Pod, Zeroable,
)]
#[repr(C)]
pub struct BorrowRateState {
    // Borrow rates have implied RATE_DECIMALS decimals
    pub current_rate: u64,
    pub last_update: i64,
    pub cumulative_interest: U128Split,
}

#[account(zero_copy)]
#[derive(Default, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct Custody {
    pub bump: u8,
    pub token_account_bump: u8,
    //
    // Permissions
    //
    // If false, make positions readonly/closeonly
    pub allow_trade: u8,
    pub allow_swap: u8,
    //
    pub decimals: u8,
    pub is_stable: u8,
    pub _padding: [u8; 2],
    //
    pub pool: Pubkey,
    // /!\ The position of this field matters as we use the offset to get the mint in get_custody_mint_from_account_info
    pub mint: Pubkey,
    pub token_account: Pubkey,
    // Oracle of the token (i.e jitoSOL custody, oracle is jitoSOL price)
    pub oracle: Pubkey,
    // Oracle used to trade the token (i.e jitoSOL custody, trade_oracle is SOL price)
    // Used to calculate the value of the position
    pub trade_oracle: Pubkey,
    pub pricing: PricingParams,

    pub fees: Fees,
    pub borrow_rate: BorrowRateParams,
    // All time stats
    pub collected_fees: FeesStats,
    pub volume_stats: VolumeStats,
    pub trade_stats: TradeStats,
    // Real time stats
    pub assets: Assets,
    pub long_positions: PositionsAccounting,
    pub short_positions: PositionsAccounting,
    pub borrow_rate_state: BorrowRateState,
}

#[derive(Copy, Clone, Eq, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct OraclePrice {
    pub price: u64,
    pub exponent: i32,
    pub confidence: u64,
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub enum Side {
    None = 0,
    #[default]
    Long = 1,
    Short = 2,
}

impl From<Side> for u8 {
    fn from(val: Side) -> Self {
        match val {
            Side::None => 0,
            Side::Long => 1,
            Side::Short => 2,
        }
    }
}

impl TryFrom<u8> for Side {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => Side::None,
            1 => Side::Long,
            2 => Side::Short,
            // Return an error if unknown value
            _ => anyhow::bail!("Invalid side value"),
        })
    }
}
