use anchor_lang::prelude::*;

#[error_code]
pub enum ItemNftError {
    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Invalid item type")]
    InvalidItemType,

    #[msg("Invalid NFT amount")]
    InvalidNftAmount,
}