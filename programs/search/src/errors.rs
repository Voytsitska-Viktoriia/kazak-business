use anchor_lang::prelude::*;

#[error_code]
pub enum SearchError {
    #[msg("Cooldown active")]
    CooldownActive,

    #[msg("Invalid owner")]
    InvalidOwner,
}