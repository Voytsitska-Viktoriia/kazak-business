use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("5CzubBHrnNHpqmvETAGqRBqkfSpR2jBdpGQGZGTzfG4o");

#[program]
pub mod resource_manager {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        item_prices: [u64; 4],
    ) -> Result<()> {
        initialize_game::handler(ctx, item_prices)
    }

    pub fn mint_resource(
        ctx: Context<MintResource>,
        resource_id: u8,
        amount: u64,
    ) -> Result<()> {
        mint_resource::handler(ctx, resource_id, amount)
    }

    pub fn burn_resource(
        ctx: Context<BurnResource>,
        resource_id: u8,
        amount: u64,
    ) -> Result<()> {
        burn_resource::handler(ctx, resource_id, amount)
    }
}