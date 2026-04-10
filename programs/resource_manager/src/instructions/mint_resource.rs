use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    mint_to, Mint, MintTo, TokenAccount, TokenInterface,
};

use crate::errors::ResourceManagerError;
use crate::state::GameConfig;

/// Accounts for minting resources.
#[derive(Accounts)]
#[instruction(resource_id: u8)]
pub struct MintResource<'info> {
    /// This signer is expected to be an authorized program-controlled caller.
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        seeds = [b"game-config"],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    /// CHECK: PDA authority used as mint authority for resource mints
    #[account(
        seeds = [b"resource-auth"],
        bump
    )]
    pub resource_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<MintResource>, resource_id: u8, amount: u64) -> Result<()> {
    require!(resource_id < 6, ResourceManagerError::InvalidResourceId);
    require!(amount > 0, ResourceManagerError::InvalidAmount);

    let expected_mint = ctx.accounts.game_config.resource_mints[resource_id as usize];
    require_keys_eq!(
        expected_mint,
        ctx.accounts.mint.key(),
        ResourceManagerError::InvalidMint
    );

    let seeds: &[&[u8]] = &[b"resource-auth", &[ctx.bumps.resource_authority]];
    let signer_seeds = &[seeds];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.resource_authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    mint_to(cpi_ctx, amount)?;

    Ok(())
}