use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("BnBincK5DvuXEJ1Weyw5tKGrUQJ372cooYQGuiuYnS1");

#[program]
pub mod search {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        initialize_player::handler(ctx)
    }

    pub fn search_resources(ctx: Context<SearchResources>) -> Result<()> {
        search_resources::handler(ctx)
    }
}