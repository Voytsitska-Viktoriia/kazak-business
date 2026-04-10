use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    mint_to, Mint, MintTo, TokenAccount, TokenInterface,
};

use crate::errors::MagicTokenError;
use crate::state::MagicConfig;

/// Accounts for minting MagicToken.
#[derive(Accounts)]
pub struct MintMagic<'info> {
    #[account(mut)]
    pub marketplace_caller: Signer<'info>,

    #[account(
        seeds = [b"magic-config"],
        bump = magic_config.bump
    )]
    pub magic_config: Account<'info, MagicConfig>,

    /// CHECK: PDA authority used as mint authority for MagicToken mint
    #[account(
        seeds = [b"magic-auth"],
        bump
    )]
    pub magic_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub magic_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<MintMagic>, amount: u64) -> Result<()> {
    require!(amount > 0, MagicTokenError::InvalidAmount);

    require_keys_eq!(
        ctx.accounts.magic_config.marketplace_authority,
        ctx.accounts.marketplace_caller.key(),
        MagicTokenError::UnauthorizedCaller
    );

    require_keys_eq!(
        ctx.accounts.magic_config.magic_mint,
        ctx.accounts.magic_mint.key(),
        MagicTokenError::InvalidMint
    );

    let seeds: &[&[u8]] = &[b"magic-auth", &[ctx.bumps.magic_authority]];
    let signer_seeds = &[seeds];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.magic_mint.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.magic_authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    mint_to(cpi_ctx, amount)?;

    Ok(())
}