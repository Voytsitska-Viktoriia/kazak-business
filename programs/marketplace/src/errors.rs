use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Invalid item owner")]
    InvalidItemOwner,

    #[msg("Invalid item metadata mint")]
    InvalidItemMetadataMint,

    #[msg("Invalid item type")]
    InvalidItemType,

    #[msg("Missing item price")]
    MissingItemPrice,
}