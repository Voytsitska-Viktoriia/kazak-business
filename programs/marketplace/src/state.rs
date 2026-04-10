use anchor_lang::prelude::*;

#[account]
pub struct MarketplaceConfig {
    pub admin: Pubkey,
    pub resource_manager_program: Pubkey,
    pub item_nft_program: Pubkey,
    pub magic_token_program: Pubkey,
    pub bump: u8,
}

impl MarketplaceConfig {
    pub const LEN: usize = 8 + 32 * 4 + 1;
}