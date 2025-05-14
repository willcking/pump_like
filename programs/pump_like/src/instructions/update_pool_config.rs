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
pub struct UpdatePoolConfigAdminParams {
    pub new_config_admin: Pubkey,
    pub old_config_admin: Pubkey,
}

pub fn update_pool_config_admin(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigAdminParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(pool_config.config_admin, ctx.accounts.config_admin.key());
    
    pool_config.update_pool_config_admin(params.new_config_admin.key())?;

    msg!("UpdatePoolConfigAdmin Success!");

    emit!(UpdatePoolConfigAdminEvent {
        old_config_admin: ctx.accounts.config_admin.to_account_info().key(),
        new_config_admin: params.new_config_admin.key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
    });

    Ok(())
}

