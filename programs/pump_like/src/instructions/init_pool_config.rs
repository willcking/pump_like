use anchor_lang::prelude::*;

use crate::states::pool_config::*;
use crate::constants::meme::POOL_CONFIG_SEED;

#[derive(Accounts)]
pub struct InitPoolConfig<'info> {
    #[account(mut)]
    pub config_admin: Signer<'info>,

    #[account(mut)]
    pub list_admin: UncheckedAccount<'info>,

    #[account(
        init,
        payer = config_admin,
        space = PoolConfig::LEN,
        seeds = [POOL_CONFIG_SEED.as_bytes()],
        bump,
    )]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct PoolConfigParams {
    pub swap_fee: u16,
    pub swap_fee_denominator: u16,
    pub creat_meme_fee: u64,
    pub list_to_ray_fee: u64,
    pub independant_vault: Pubkey,
}

pub fn init_pool_config(ctx: Context<InitPoolConfig>, params: PoolConfigParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_init()?;
    let bump = ctx.bumps.pool_config;
    pool_config.initialize(
        bump,
        params.swap_fee,
        params.swap_fee_denominator,
        params.creat_meme_fee,
        params.list_to_ray_fee,
        params.independant_vault,
        ctx.accounts.config_admin.to_account_info().key.clone(),
        ctx.accounts.list_admin.to_account_info().key(),
    )?;
    msg!("Pool config initialized");


    emit!(CreatePoolConfigEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        list_admin: ctx.accounts.list_admin.to_account_info().key(),
        swap_fee: params.swap_fee,
        swap_fee_denominator: params.swap_fee_denominator,
        creat_meme_fee: params.creat_meme_fee,
        list_to_ray_fee: params.list_to_ray_fee,
        independant_vault: params.independant_vault,
    });
    Ok(())
}