use anchor_lang::prelude::*;

use crate::states::pool_config::*;
use crate::constants::POOL_CONFIG_SEED;

#[derive(Accounts)]
pub struct UpdatePoolConfig<'info> {
    #[account(mut)]
    pub config_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED.as_bytes()],
        bump,
    )]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigParams {
    pub swap_fee: u16,
}