use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    burn, Burn, Mint, TokenAccount, TokenInterface,
};

use crate::errors::ItemNftError;
use crate::state::ItemMetadata;

/// Accounts for burning an item NFT.
#[derive(Accounts)]
pub struct BurnItem<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub item_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = owner_item_account.owner == owner.key() @ ItemNftError::InvalidTokenOwner,
        constraint = owner_item_account.mint == item_mint.key() @ ItemNftError::InvalidMint
    )]
    pub owner_item_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"item-metadata", item_mint.key().as_ref()],
        bump = item_metadata.bump
    )]
    pub item_metadata: Account<'info, ItemMetadata>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<BurnItem>) -> Result<()> {
    require!(
        ctx.accounts.owner_item_account.amount == 1,
        ItemNftError::InvalidNftAmount
    );

    let cpi_accounts = Burn {
        mint: ctx.accounts.item_mint.to_account_info(),
        from: ctx.accounts.owner_item_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    burn(cpi_ctx, 1)?;

    Ok(())
}