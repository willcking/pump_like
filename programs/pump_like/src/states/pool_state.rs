use anchor_lang::prelude::*;

#[account]
pub struct PoolState {
    pub bump: u8,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
    pub meme_metadata: Pubkey,
    pub meme_supply: u64,
    pub meme_decimals: u8,
    
}