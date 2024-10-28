pub use {
    crate::{pda::*, types::*},
    anchor_lang::prelude::*,
    std::str::FromStr,
};

pub mod liquidation_price;
pub mod math;
pub mod pda;
pub mod types;

declare_id!("13gDzEXCdocbj8iAiqrScGo47NiSuYENGsRqi3SEAwet");

pub static SPL_TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub static SPL_ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub static SABLIER_THREAD_PROGRAM_ID: Pubkey =
    pubkey!("sabGLGXfBiUCkwtprPMtatG6tCNxhcWWs1hjQAvDqEE");
pub static SABLIER_NETWORK_PROGRAM_ID: Pubkey =
    pubkey!("H6CTDj7ewA6PT1jGt2WTnkhWb2RwwuoQkAsgzRuSsnaV");
pub static SPL_GOVERNANCE_PROGRAM_ID: Pubkey =
    pubkey!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

pub static USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub static BONK_MINT: Pubkey = pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");
pub static JITO_MINT: Pubkey = pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
pub static WBTC_MINT: Pubkey = pubkey!("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh");

pub static ADRENA_PROGRAM_ID: Pubkey = pubkey!("13gDzEXCdocbj8iAiqrScGo47NiSuYENGsRqi3SEAwet");
pub static CORTEX_ID: Pubkey = pubkey!("Dhz8Ta79hgyUbaRcu7qHMnqMfY47kQHfHt2s42D9dC4e");
pub static ADX_MINT: Pubkey = pubkey!("AuQaustGiaqxRvj2gtCdrd22PBzTn8kM3kEPEkZCtuDw");
pub static ALP_MINT: Pubkey = pubkey!("4yCLi5yWGzpTWMQ1iWHG5CrGYAdBkhyEdsuSugjDUqwj");
pub static MAIN_POOL_ID: Pubkey = pubkey!("4bQRutgDJs6vuh6ZcWaPVXiQaBzbHketjbCDjL4oRN34");

pub mod main_pool {
    use super::*;
    pub static USDC_CUSTODY_ID: Pubkey = pubkey!("Dk523LZeDQbZtUwPEBjFXCd2Au1tD7mWZBJJmcgHktNk");
    pub static BONK_CUSTODY_ID: Pubkey = pubkey!("8aJuzsgjxBnvRhDcfQBD7z4CUj7QoPEpaNwVd7KqsSk5");
    pub static JITOSOL_CUSTODY_ID: Pubkey = pubkey!("GZ9XfWwgTRhkma2Y91Q9r1XKotNXYjBnKKabj19rhT71");
    pub static WBTC_CUSTODY_ID: Pubkey = pubkey!("GFu3qS22mo6bAjg4Lr5R7L8pPgHq6GvbjJPKEHkbbs2c");
}

#[program]
mod adrena_abi {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(clippy::too_many_arguments)]

    use super::*;

    // ========== MARGIN ==========

    pub(crate) fn close_position_long(
        cx: Context<ClosePositionLong>,
        params: ClosePositionLongParams,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_position_short(
        cx: Context<ClosePositionShort>,
        params: ClosePositionShortParams,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn liquidate_long(
        cx: Context<LiquidateLong>,
        params: LiquidateLongParams,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn liquidate_short(
        cx: Context<LiquidateShort>,
        params: LiquidateShortParams,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cleanup_position_stop_loss(cx: Context<CleanupPositionStopLoss>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cleanup_position_take_profit(
        cx: Context<CleanupPositionTakeProfit>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn resolve_staking_round(cx: Context<ResolveStakingRound>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClosePositionLong<'info> {
    /// #1
    #[account(mut)]
    pub caller: Signer<'info>,
    /// #2
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// #3
    #[account(mut)]
    pub receiving_account: AccountInfo<'info>,
    /// #4
    pub transfer_authority: AccountInfo<'info>,
    /// #5
    #[account(mut)]
    pub lm_staking: AccountLoader<'info, Staking>,
    /// #6
    #[account(mut)]
    pub lp_staking: AccountLoader<'info, Staking>,
    /// #7
    #[account(mut)]
    pub cortex: AccountLoader<'info, Cortex>,
    /// #8
    #[account(mut)]
    pub pool: AccountLoader<'info, Pool>,
    /// #9
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #10
    #[account(mut)]
    pub staking_reward_token_custody: AccountLoader<'info, Custody>,
    /// #11
    pub staking_reward_token_custody_oracle: AccountInfo<'info>,
    /// #12
    #[account(mut)]
    pub staking_reward_token_custody_token_account: AccountInfo<'info>,
    /// #13
    #[account(mut)]
    pub custody: AccountLoader<'info, Custody>,
    /// #14
    pub custody_oracle: AccountInfo<'info>,
    /// #15
    pub custody_trade_oracle: AccountInfo<'info>,
    /// #16
    #[account(mut)]
    pub custody_token_account: AccountInfo<'info>,
    /// #17
    #[account(mut)]
    pub lm_staking_reward_token_vault: AccountInfo<'info>,
    /// #18
    #[account(mut)]
    pub lp_staking_reward_token_vault: AccountInfo<'info>,
    /// #19
    #[account(mut)]
    pub lp_token_mint: AccountInfo<'info>,
    /// #20
    #[account(mut)]
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// #21
    #[account(mut)]
    pub user_profile: Option<AccountLoader<'info, UserProfile>>,
    /// #22
    #[account(mut)]
    pub take_profit_thread: UncheckedAccount<'info>,
    /// #23
    #[account(mut)]
    pub stop_loss_thread: UncheckedAccount<'info>,
    /// #24
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    token_program: AccountInfo<'info>,
    /// #25
    #[account(address = ADRENA_PROGRAM_ID)]
    adrena_program: AccountInfo<'info>,
    /// #26
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClosePositionShort<'info> {
    /// #1
    #[account(mut)]
    pub caller: Signer<'info>,
    /// #2
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// #3
    #[account(mut)]
    pub receiving_account: AccountInfo<'info>,
    /// #4
    pub transfer_authority: AccountInfo<'info>,
    /// #5
    #[account(mut)]
    pub lm_staking: AccountLoader<'info, Staking>,
    /// #6
    #[account(mut)]
    pub lp_staking: AccountLoader<'info, Staking>,
    /// #7
    #[account(mut)]
    pub cortex: AccountLoader<'info, Cortex>,
    /// #8
    #[account(mut)]
    pub pool: AccountLoader<'info, Pool>,
    /// #9
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #10
    #[account(mut)]
    pub staking_reward_token_custody: AccountLoader<'info, Custody>,
    /// #11
    pub staking_reward_token_custody_oracle: AccountInfo<'info>,
    /// #12
    #[account(mut)]
    pub staking_reward_token_custody_token_account: AccountInfo<'info>,
    /// #13
    #[account(mut)]
    pub custody: AccountLoader<'info, Custody>,
    /// #14
    pub custody_trade_oracle: AccountInfo<'info>,
    /// #15
    #[account(mut)]
    pub collateral_custody: AccountLoader<'info, Custody>,
    /// #16
    pub collateral_custody_oracle: AccountInfo<'info>,
    /// #17
    #[account(mut)]
    pub collateral_custody_token_account: AccountInfo<'info>,
    /// #18
    #[account(mut)]
    pub lm_staking_reward_token_vault: AccountInfo<'info>,
    /// #19
    #[account(mut)]
    pub lp_staking_reward_token_vault: AccountInfo<'info>,
    /// #20
    #[account(mut)]
    pub lp_token_mint: AccountInfo<'info>,
    /// #21
    #[account(mut)]
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// #22
    #[account(mut)]
    pub user_profile: Option<AccountLoader<'info, UserProfile>>,
    /// #23
    #[account(mut)]
    pub take_profit_thread: UncheckedAccount<'info>,
    /// #24
    #[account(mut)]
    pub stop_loss_thread: UncheckedAccount<'info>,
    /// #25
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    token_program: AccountInfo<'info>,
    /// #26
    #[account(address = ADRENA_PROGRAM_ID)]
    adrena_program: AccountInfo<'info>,
    /// #27
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateShort<'info> {
    /// #1
    #[account(mut)]
    pub signer: Signer<'info>,
    /// #2
    #[account(mut)]
    pub receiving_account: AccountInfo<'info>,
    /// #3
    pub transfer_authority: AccountInfo<'info>,
    /// #4
    #[account(mut)]
    pub lm_staking: AccountLoader<'info, Staking>,
    /// #5
    #[account(mut)]
    pub lp_staking: AccountLoader<'info, Staking>,
    /// #6
    #[account(mut)]
    pub cortex: AccountLoader<'info, Cortex>,
    /// #7
    #[account(mut)]
    pub pool: AccountLoader<'info, Pool>,
    /// #8
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #9
    #[account(mut)]
    pub staking_reward_token_custody: AccountLoader<'info, Custody>,
    /// #10
    pub staking_reward_token_custody_oracle: AccountInfo<'info>,
    /// #11
    #[account(mut)]
    pub staking_reward_token_custody_token_account: AccountInfo<'info>,
    /// #12
    #[account(mut)]
    pub custody: AccountLoader<'info, Custody>,
    /// #13
    pub custody_trade_oracle: AccountInfo<'info>,
    /// #14
    #[account(mut)]
    pub collateral_custody: AccountLoader<'info, Custody>,
    /// #15
    pub collateral_custody_oracle: AccountInfo<'info>,
    /// #16
    #[account(mut)]
    pub collateral_custody_token_account: AccountInfo<'info>,
    /// #17
    #[account(mut)]
    pub lm_staking_reward_token_vault: AccountInfo<'info>,
    /// #18
    #[account(mut)]
    pub lp_staking_reward_token_vault: AccountInfo<'info>,
    /// #19
    #[account(mut)]
    pub lp_token_mint: AccountInfo<'info>,
    /// #20
    #[account(mut)]
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// #21
    #[account(mut)]
    pub user_profile: Option<AccountLoader<'info, UserProfile>>,
    /// #22
    #[account(mut)]
    pub take_profit_thread: UncheckedAccount<'info>,
    /// #23
    #[account(mut)]
    pub stop_loss_thread: UncheckedAccount<'info>,
    /// #24
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    token_program: AccountInfo<'info>,
    /// #25
    #[account(address = ADRENA_PROGRAM_ID)]
    adrena_program: AccountInfo<'info>,
    /// #26
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LiquidateLong<'info> {
    /// #1
    #[account(mut)]
    pub signer: Signer<'info>,
    /// #2
    #[account(mut)]
    pub receiving_account: AccountInfo<'info>,
    /// #3
    pub transfer_authority: AccountInfo<'info>,
    /// #4
    #[account(mut)]
    pub lm_staking: AccountLoader<'info, Staking>,
    /// #5
    #[account(mut)]
    pub lp_staking: AccountLoader<'info, Staking>,
    /// #6
    #[account(mut)]
    pub cortex: AccountLoader<'info, Cortex>,
    /// #7
    #[account(mut)]
    pub pool: AccountLoader<'info, Pool>,
    /// #8
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #9
    #[account(mut)]
    pub staking_reward_token_custody: AccountLoader<'info, Custody>,
    /// #10
    pub staking_reward_token_custody_oracle: AccountInfo<'info>,
    /// #11
    #[account(mut)]
    pub staking_reward_token_custody_token_account: AccountInfo<'info>,
    /// #12
    #[account(mut)]
    pub custody: AccountLoader<'info, Custody>,
    /// #13
    pub custody_oracle: AccountInfo<'info>,
    /// #14
    pub custody_trade_oracle: AccountInfo<'info>,
    /// #15
    #[account(mut)]
    pub custody_token_account: AccountInfo<'info>,
    /// #16
    #[account(mut)]
    pub lm_staking_reward_token_vault: AccountInfo<'info>,
    /// #17
    #[account(mut)]
    pub lp_staking_reward_token_vault: AccountInfo<'info>,
    /// #18
    #[account(mut)]
    pub lp_token_mint: AccountInfo<'info>,
    /// #19
    #[account(mut)]
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// #20
    #[account(mut)]
    pub user_profile: Option<AccountLoader<'info, UserProfile>>,
    /// #21
    #[account(mut)]
    pub take_profit_thread: UncheckedAccount<'info>,
    /// #22
    #[account(mut)]
    pub stop_loss_thread: UncheckedAccount<'info>,
    /// #23
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    token_program: AccountInfo<'info>,
    /// #24
    #[account(address = ADRENA_PROGRAM_ID)]
    adrena_program: AccountInfo<'info>,
    /// #25
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanupPositionStopLoss<'info> {
    /// #1
    #[account(mut)]
    pub caller: Signer<'info>,
    /// #2
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// #3
    pub transfer_authority: AccountInfo<'info>,
    /// #4
    pub cortex: AccountLoader<'info, Cortex>,
    /// #5
    pub pool: AccountLoader<'info, Pool>,
    /// #6
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #7
    pub custody: AccountLoader<'info, Custody>,
    /// #8
    #[account(mut)]
    pub stop_loss_thread: AccountInfo<'info>,
    /// #9
    #[account(mut)]
    pub take_profit_thread: UncheckedAccount<'info>,
    /// #10
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CleanupPositionTakeProfit<'info> {
    /// #1
    #[account(mut)]
    pub caller: Signer<'info>,
    /// #2
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    /// #3
    pub transfer_authority: AccountInfo<'info>,
    /// #4
    pub cortex: AccountLoader<'info, Cortex>,
    /// #5
    pub pool: AccountLoader<'info, Pool>,
    /// #6
    #[account(mut)]
    pub position: AccountLoader<'info, Position>,
    /// #7
    pub custody: AccountLoader<'info, Custody>,
    /// #8
    #[account(mut)]
    pub take_profit_thread: AccountInfo<'info>,
    /// #9
    #[account(mut)]
    pub stop_loss_thread: UncheckedAccount<'info>,
    /// #10
    #[account(address = SABLIER_THREAD_PROGRAM_ID)]
    sablier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub(crate) struct ResolveStakingRound<'info> {
    /// #1
    #[account(mut)]
    pub caller: Signer<'info>,

    /// #2
    #[account(mut)]
    pub payer: Signer<'info>,

    /// #3
    /// CHECKS: only for CPI
    #[account(mut)]
    pub staking_staked_token_vault: UncheckedAccount<'info>,

    /// #4
    /// CHECKS: only for CPI
    #[account(mut)]
    pub staking_reward_token_vault: UncheckedAccount<'info>,

    /// #5
    /// CHECKS: only for CPI
    #[account(mut)]
    pub staking_lm_reward_token_vault: UncheckedAccount<'info>,

    /// #6
    /// CHECKS: only for CPI
    pub transfer_authority: UncheckedAccount<'info>,

    /// #7
    /// CHECKS: only for CPI
    #[account(mut)]
    pub staking: UncheckedAccount<'info>,

    /// #8
    /// CHECKS: only for CPI
    #[account(mut)]
    pub cortex: UncheckedAccount<'info>,

    /// #9
    /// CHECKS: only for CPI
    #[account(mut)]
    pub lm_token_mint: UncheckedAccount<'info>,

    /// #10
    /// CHECKS: only for CPI
    pub fee_redistribution_mint: UncheckedAccount<'info>,

    /// #11
    /// CHECKS: only for CPI
    pub adrena_program: UncheckedAccount<'info>,

    /// #12
    /// CHECKS: only for CPI
    pub system_program: UncheckedAccount<'info>,

    /// #13
    /// CHECKS: only for CPI
    pub token_program: UncheckedAccount<'info>,
}
