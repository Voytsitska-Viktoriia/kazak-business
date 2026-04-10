use anchor_lang::prelude::*;

#[account]
pub struct ItemMetadata {
    pub owner: Pubkey,
    pub mint: Pubkey,       
    pub item_type: u8,
    pub bump: u8,           
}

impl ItemMetadata {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1; // 
}