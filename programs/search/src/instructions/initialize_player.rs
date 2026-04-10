use anchor_lang::prelude::*;

use crate::state::Player;

/// Accounts for creating a player PDA.
#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Player::LEN,
        seeds = [b"player", owner.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePlayer>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    player.owner = ctx.accounts.owner.key();
    player.last_search_timestamp = 0;
    player.bump = ctx.bumps.player;

    Ok(())
}