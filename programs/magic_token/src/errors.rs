use anchor_lang::prelude::*;

#[error_code]
pub enum MagicTokenError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Unauthorized caller")]
    UnauthorizedCaller,

    #[msg("Invalid mint")]
    InvalidMint,
}