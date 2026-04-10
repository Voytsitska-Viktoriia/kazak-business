use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("D3yh8ZJxF6T7Rb3gXdkduUjaiM7kgug9BsFPeqP6W2PG");

#[program]
pub mod item_nft {
    use super::*;

    pub fn mint_item(ctx: Context<MintItem>) -> Result<()> {
        mint_item::handler(ctx, 0) // просто 0
    }

    pub fn burn_item(ctx: Context<BurnItem>) -> Result<()> {
        burn_item::handler(ctx)
    }
}