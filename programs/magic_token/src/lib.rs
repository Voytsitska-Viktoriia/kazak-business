use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("A2gokn42YMW7iFwgVc3sG2QMvJVF2Gmh4Zr1AQcmNYra");

#[program]
pub mod magic_token {
    use super::*;

    pub fn initialize_magic_config(
        ctx: Context<InitializeMagicConfig>,
    ) -> Result<()> {
        initialize_magic_config::handler(ctx, Pubkey::default()) // 🔥 ФІКС
    }

    pub fn mint_magic(ctx: Context<MintMagic>) -> Result<()> {
        mint_magic::handler(ctx, 1) // 🔥 ФІКС
    }
}