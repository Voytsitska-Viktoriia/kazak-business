use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

/// 🔥 тимчасовий handler щоб зібралось
pub fn handler(_ctx: Context<CraftItem>, _item_type: u8) -> Result<()> {
    Ok(())
}