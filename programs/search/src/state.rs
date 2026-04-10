use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub owner: Pubkey,
    pub last_search_timestamp: i64, // 🔥 ВАЖЛИВО
    pub bump: u8,
}

impl Player {
    pub const LEN: usize = 8 + 32 + 8 + 1;
}