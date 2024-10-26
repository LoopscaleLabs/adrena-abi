use {
    crate::{Side, SABLIER_THREAD_PROGRAM_ID},
    anchor_lang::prelude::*,
};

#[constant]
pub const SEED_THREAD: &[u8] = b"thread";

#[constant]
pub const SEED_CONFIG: &[u8] = b"config";

pub fn get_transfer_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&["transfer_authority".as_ref()], &crate::id())
}

pub fn get_cortex_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&["cortex".as_ref()], &crate::id())
}

pub fn get_vest_registry_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&["vest_registry".as_ref()], &crate::id())
}

pub fn get_genesis_lock_pda(pool_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["genesis_lock".as_ref(), pool_pda.as_ref()], &crate::id())
}

pub fn get_lm_token_mint_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&["lm_token_mint".as_ref()], &crate::id())
}

pub fn get_sablier_thread_pda(
    thread_authority: &Pubkey,
    thread_id: Vec<u8>,
    domain: Option<Vec<u8>>,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.as_ref(),
            thread_id.as_slice(),
            domain.unwrap_or_default().as_slice(),
        ],
        &SABLIER_THREAD_PROGRAM_ID,
    )
}

pub fn get_governance_token_mint_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&["governance_token_mint".as_ref()], &crate::id())
}

pub fn get_user_staking_pda(owner: &Pubkey, staking_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            "user_staking".as_ref(),
            owner.as_ref(),
            staking_pda.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn get_staking_pda(staked_token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &["staking".as_ref(), staked_token_mint.as_ref()],
        &crate::id(),
    )
}

pub fn get_staking_staked_token_vault_pda(staking_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &["staking_staked_token_vault".as_ref(), staking_pda.as_ref()],
        &crate::id(),
    )
}

pub fn get_staking_reward_token_vault_pda(staking_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &["staking_reward_token_vault".as_ref(), staking_pda.as_ref()],
        &crate::id(),
    )
}

pub fn get_user_profile_pda(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["user_profile".as_ref(), user.as_ref()], &crate::id())
}

pub fn get_staking_lm_reward_token_vault_pda(staking_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            "staking_lm_reward_token_vault".as_ref(),
            staking_pda.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn get_program_data_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[crate::id().as_ref()],
        &anchor_lang::solana_program::bpf_loader_upgradeable::id(),
    )
}

pub fn get_pool_pda(name: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["pool".as_ref(), name.as_bytes()], &crate::id())
}

pub fn get_vest_pda(owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["vest".as_ref(), owner.as_ref()], &crate::id())
}

pub fn get_lp_token_mint_pda(pool_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["lp_token_mint".as_ref(), pool_pda.as_ref()], &crate::id())
}

pub fn get_custody_pda(pool_pda: &Pubkey, custody_token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            "custody".as_ref(),
            pool_pda.as_ref(),
            custody_token_mint.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn get_position_pda(
    owner: &Pubkey,
    pool_pda: &Pubkey,
    custody_pda: &Pubkey,
    side: Side,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            "position".as_ref(),
            owner.as_ref(),
            pool_pda.as_ref(),
            custody_pda.as_ref(),
            &[side as u8],
        ],
        &crate::id(),
    )
}

pub fn get_custody_token_account_pda(
    pool_pda: &Pubkey,
    custody_token_mint: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            "custody_token_account".as_ref(),
            pool_pda.as_ref(),
            custody_token_mint.as_ref(),
        ],
        &crate::id(),
    )
}
