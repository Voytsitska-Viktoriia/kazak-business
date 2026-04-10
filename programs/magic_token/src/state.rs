use anchor_lang::prelude::*;

#[account]
pub struct MagicConfig {
    pub admin: Pubkey,
    pub magic_mint: Pubkey,
    pub marketplace_authority: Pubkey,
    pub bump: u8,
}

impl MagicConfig {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1;
}