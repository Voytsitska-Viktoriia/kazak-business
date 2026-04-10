use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("9LWJAeYhzQRhiBEsMdsBg6yKDPJLvcGbjtzr9arFU4sn");

#[program]
pub mod crafting {
    use super::*;

    pub fn craft_item(ctx: Context<CraftItem>) -> Result<()> {
        craft_item::handler(ctx, 0) // 🔥 просто 0
    }
}