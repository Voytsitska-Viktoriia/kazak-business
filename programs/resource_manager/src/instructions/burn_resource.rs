use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    burn, Burn, Mint, TokenAccount, TokenInterface,
};

use crate::errors::ResourceManagerError;
use crate::state::GameConfig;

/// Accounts for burning resources from a player's token account.
#[derive(Accounts)]
#[instruction(resource_id: u8)]
pub struct BurnResource<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        seeds = [b"game-config"],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = player_token_account.owner == player.key() @ ResourceManagerError::InvalidTokenOwner
    )]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<BurnResource>, resource_id: u8, amount: u64) -> Result<()> {
    require!(resource_id < 6, ResourceManagerError::InvalidResourceId);
    require!(amount > 0, ResourceManagerError::InvalidAmount);

    let expected_mint = ctx.accounts.game_config.resource_mints[resource_id as usize];
    require_keys_eq!(
        expected_mint,
        ctx.accounts.mint.key(),
        ResourceManagerError::InvalidMint
    );

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.player_token_account.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    burn(cpi_ctx, amount)?;

    Ok(())
}