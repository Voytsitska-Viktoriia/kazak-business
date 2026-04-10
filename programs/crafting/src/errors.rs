use anchor_lang::prelude::*;

#[error_code]
pub enum CraftingError {
    #[msg("Invalid item type")]
    InvalidItemType,

    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Insufficient resources")]
    InsufficientResources,
}