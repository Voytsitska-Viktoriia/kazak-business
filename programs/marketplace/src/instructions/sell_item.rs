use anchor_lang::prelude::*;

use crate::errors::MarketplaceError;
use crate::state::MarketplaceConfig;

#[derive(Accounts)]
pub struct SellItem<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub marketplace_config: Account<'info, MarketplaceConfig>,
}

pub fn handler(_ctx: Context<SellItem>) -> Result<()> {
    Ok(())
}