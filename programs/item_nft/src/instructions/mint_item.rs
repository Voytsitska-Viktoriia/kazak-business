use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    mint_to, Mint, MintTo, TokenAccount, TokenInterface,
};

use crate::errors::ItemNftError;
use crate::state::ItemMetadata;

/// Accounts for minting a new item NFT.
#[derive(Accounts)]
#[instruction(item_type: u8)]
pub struct MintItem<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: PDA authority that controls item NFT minting.
    #[account(
        seeds = [b"item-auth"],
        bump
    )]
    pub item_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub item_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = player_item_account.owner == player.key() @ ItemNftError::InvalidTokenOwner,
        constraint = player_item_account.mint == item_mint.key() @ ItemNftError::InvalidMint
    )]
    pub player_item_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = player,
        space = ItemMetadata::LEN,
        seeds = [b"item-metadata", item_mint.key().as_ref()],
        bump
    )]
    pub item_metadata: Account<'info, ItemMetadata>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MintItem>, item_type: u8) -> Result<()> {
    require!(item_type <= 3, ItemNftError::InvalidItemType);

    let metadata = &mut ctx.accounts.item_metadata;
    metadata.item_type = item_type;
    metadata.owner = ctx.accounts.player.key();
    metadata.mint = ctx.accounts.item_mint.key();
    metadata.bump = ctx.bumps.item_metadata;

    let seeds: &[&[u8]] = &[b"item-auth", &[ctx.bumps.item_authority]];
    let signer_seeds = &[seeds];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.item_mint.to_account_info(),
        to: ctx.accounts.player_item_account.to_account_info(),
        authority: ctx.accounts.item_authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    mint_to(cpi_ctx, 1)?;

    Ok(())
}