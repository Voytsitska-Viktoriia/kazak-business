use anchor_lang::prelude::*;

use crate::errors::SearchError;
use crate::state::Player;

/// Event emitted after a successful search.
#[event]
pub struct SearchResult {
    pub resources: [u8; 3],
    pub timestamp: i64,
}

/// Accounts for searching resources with a 60-second cooldown.
#[derive(Accounts)]
pub struct SearchResources<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", owner.key().as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,
}

pub fn handler(ctx: Context<SearchResources>) -> Result<()> {
    let clock = Clock::get()?;
    let player = &mut ctx.accounts.player;

    require_keys_eq!(
        player.owner,
        ctx.accounts.owner.key(),
        SearchError::InvalidOwner
    );

    if player.last_search_timestamp != 0 {
        require!(
            clock.unix_timestamp - player.last_search_timestamp >= 60,
            SearchError::CooldownActive
        );
    }

    player.last_search_timestamp = clock.unix_timestamp;

    // Deterministic pseudo-randomness suitable for tests/devnet.
    let slot = clock.slot;
    let r1 = (slot % 6) as u8;
    let r2 = ((slot + 1) % 6) as u8;
    let r3 = ((slot + 2) % 6) as u8;

    emit!(SearchResult {
        resources: [r1, r2, r3],
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}