use anchor_lang::prelude::*;

use crate::state::MagicConfig;

/// Accounts for initializing MagicToken configuration.
#[derive(Accounts)]
pub struct InitializeMagicConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = MagicConfig::LEN,
        seeds = [b"magic-config"],
        bump
    )]
    pub magic_config: Account<'info, MagicConfig>,

    /// CHECK: validated by storing as immutable config field
    pub magic_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMagicConfig>,
    marketplace_authority: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.magic_config;
    config.admin = ctx.accounts.admin.key();
    config.magic_mint = ctx.accounts.magic_mint.key();
    config.marketplace_authority = marketplace_authority;
    config.bump = ctx.bumps.magic_config;

    Ok(())
}