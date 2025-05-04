use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Default, Debug)]

pub struct PoolConfig {
    pub bump: u8,

    pub swap_fee: u16,
    pub swap_fee_denominator: u16,

    pub create_meme_pool_fee: u64,
    pub list_to_raydium_fee: u64,

    pub fee_vault: Pubkey,

    pub config_authority: Pubkey,
    pub list_authority: Pubkey,
}

impl PoolConfig {
    pub const LEN: usize = 8 + 1 + 2 + 2 + 8 + 8 + 32 + 32 + 32 + 512;
    
    pub fn get_list_to_ray_fee(&self) -> u64 {
        self.list_to_raydium_fee
    }
}