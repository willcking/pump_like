use anchor_lang::prelude::*;
use crate::constants::meme::POOL_SEED;

#[account]
pub struct PoolState {
    pub bump: u8,

    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,

    pub rent_amount: u64,

    pub is_ready_to_list: bool,

    pub pool_config: Pubkey,
}

impl PoolState {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 8 + 1 + 32 + 512;

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            &POOL_SEED.as_bytes(),
            self.meme_mint.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(&self.seeds(), &crate::id()).unwrap()
    }

    pub fn initialize(
        &mut self,
        bump: u8,
        meme_mint: Pubkey,
        meme_vault: Pubkey,
        pool_config: Pubkey,
    ) -> Result<()> {
        self.bump = [bump];
        self.meme_mint = meme_mint;
        self.meme_vault = meme_vault;
        self.is_ready_to_list = false;
        self.pool_config = pool_config;
        Ok(())
    }

    pub fn set_is_ready_to_list(&mut self, is_ready_to_list: bool) -> Result<()> {
        self.is_ready_to_list = is_ready_to_list;
        Ok(())
    }

    pub fn get_is_ready_to_list(&self) -> bool {
        self.is_ready_to_list
    }

    pub fn set_rent_amount(&mut self, amount: u64) -> Result<()> {
        self.rent_amount = amount;
        Ok(())
    }

    pub fn get_rent_amount(&self) -> u64 {
        self.rent_amount
    }
}