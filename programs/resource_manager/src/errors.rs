use anchor_lang::prelude::*;

#[error_code]
pub enum ResourceManagerError {
    #[msg("Unauthorized caller")]
    UnauthorizedCaller,

    #[msg("Invalid resource id")]
    InvalidResourceId,

    #[msg("Invalid mint for the given resource id")]
    InvalidMint,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Player token account owner mismatch")]
    InvalidTokenOwner,
}