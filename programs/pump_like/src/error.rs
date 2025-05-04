use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("SignerIsNotAuthority")]
    SignerIsNotAuthority,
    #[msg("InsufficientBalance")]
    InsufficientBalance,
    #[msg("AddressNotExist")]
    AddressNotExist,
    #[msg("PoolListedToRaydium")]
    PoolListedToRaydium,
    #[msg("PoolReadyToList")]
    PoolReadyToList,
    #[msg("PoolNotReadyToList")]
    PoolNotReadyToList,
}