use anchor_lang::prelude::*;

use crate::state::MarketplaceConfig;

/// Accounts for initializing marketplace configuration.
#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = MarketplaceConfig::LEN,
        seeds = [b"marketplace-config"],
        bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    /// CHECK: stored as config only
    pub resource_manager_program: UncheckedAccount<'info>,

    /// CHECK: stored as config only
    pub item_nft_program: UncheckedAccount<'info>,

    /// CHECK: stored as config only
    pub magic_token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMarketplace>) -> Result<()> {
    let config = &mut ctx.accounts.marketplace_config;
    config.admin = ctx.accounts.admin.key();
    config.resource_manager_program = ctx.accounts.resource_manager_program.key();
    config.item_nft_program = ctx.accounts.item_nft_program.key();
    config.magic_token_program = ctx.accounts.magic_token_program.key();
    config.bump = ctx.bumps.marketplace_config;

    Ok(())
}