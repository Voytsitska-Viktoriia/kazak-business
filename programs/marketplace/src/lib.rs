use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

// 🔥 ОЦЕ КРИТИЧНО
pub use instructions::*;

declare_id!("6ZsDYbMP5R6o8oAeqU6zwJvdLzTc96bTLvLjiRQNn59u");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>) -> Result<()> {
        initialize_marketplace::handler(ctx)
    }

    pub fn sell_item(ctx: Context<SellItem>) -> Result<()> {
        sell_item::handler(ctx)
    }
}