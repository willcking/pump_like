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

    pub fn initialize(
        &mut self,
        bump: u8,
        swap_fee: u16,
        swap_fee_denominator: u16,
        create_meme_pool_fee: u64,
        list_to_raydium_fee: u64,
        fee_vault: Pubkey,
        config_authority: Pubkey,
        list_authority: Pubkey,
    ) -> Result<()> {
        self.bump = bump;
        self.swap_fee = swap_fee;
        self.swap_fee_denominator = swap_fee_denominator;
        self.create_meme_pool_fee = create_meme_pool_fee;
        self.list_to_raydium_fee = list_to_raydium_fee;
        self.fee_vault = fee_vault;
        self.config_authority = config_authority;
        self.list_authority = list_authority;
        Ok(())
    }
    
    pub fn get_list_to_ray_fee(&self) -> u64 {
        self.list_to_raydium_fee
    }
}