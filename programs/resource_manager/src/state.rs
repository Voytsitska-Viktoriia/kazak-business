use anchor_lang::prelude::*;

/// Stores the global game configuration.
#[account]
pub struct GameConfig {
    pub admin: Pubkey,
    pub resource_mints: [Pubkey; 6],
    pub magic_token_mint: Pubkey,
    pub item_prices: [u64; 4],
    pub bump: u8,
}

impl GameConfig {
    pub const LEN: usize = 8 + 32 + (32 * 6) + 32 + (8 * 4) + 1;
}