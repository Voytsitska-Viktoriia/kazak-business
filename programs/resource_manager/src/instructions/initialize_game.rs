use anchor_lang::prelude::*;

use crate::state::GameConfig;

/// Accounts for initializing the game configuration.
#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = GameConfig::LEN,
        seeds = [b"game-config"],
        bump
    )]
    pub game_config: Account<'info, GameConfig>,

    /// CHECK: resource mint 0 address is validated by being stored in config
    pub resource_mint_0: UncheckedAccount<'info>,

    /// CHECK: resource mint 1 address is validated by being stored in config
    pub resource_mint_1: UncheckedAccount<'info>,

    /// CHECK: resource mint 2 address is validated by being stored in config
    pub resource_mint_2: UncheckedAccount<'info>,

    /// CHECK: resource mint 3 address is validated by being stored in config
    pub resource_mint_3: UncheckedAccount<'info>,

    /// CHECK: resource mint 4 address is validated by being stored in config
    pub resource_mint_4: UncheckedAccount<'info>,

    /// CHECK: resource mint 5 address is validated by being stored in config
    pub resource_mint_5: UncheckedAccount<'info>,

    /// CHECK: magic token mint address is validated by being stored in config
    pub magic_token_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeGame>, item_prices: [u64; 4]) -> Result<()> {
    let game_config = &mut ctx.accounts.game_config;

    game_config.admin = ctx.accounts.admin.key();
    game_config.resource_mints = [
        ctx.accounts.resource_mint_0.key(),
        ctx.accounts.resource_mint_1.key(),
        ctx.accounts.resource_mint_2.key(),
        ctx.accounts.resource_mint_3.key(),
        ctx.accounts.resource_mint_4.key(),
        ctx.accounts.resource_mint_5.key(),
    ];
    game_config.magic_token_mint = ctx.accounts.magic_token_mint.key();
    game_config.item_prices = item_prices;
    game_config.bump = ctx.bumps.game_config;

    Ok(())
}