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

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigSwapFeeParams {
    pub swap_fee: u16,
    pub swap_fee_denominator: u16,
}

pub fn update_pool_config_swap_fee(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigSwapFeeParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(ctx.accounts.config_admin.to_account_info().key(), pool_config.config_admin.key());
    
    pool_config.update_pool_config_swap_fee(params.swap_fee, params.swap_fee_denominator)?;

    msg!("UpdatePoolConfigSwapFee Success!");

    emit!(UpdatePoolConfigSwapFeeEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
        swap_fee: params.swap_fee,
        swap_fee_denominator: params.swap_fee_denominator,
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigCreateMemePoolFeeParams {
    pub create_meme_pool_fee: u64,
}

pub fn update_pool_config_create_meme_pool_fee(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigCreateMemePoolFeeParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(ctx.accounts.config_admin.to_account_info().key(), pool_config.config_admin.key());
    
    pool_config.update_pool_config_create_meme_pool_fee(params.create_meme_pool_fee)?;

    msg!("UpdatePoolConfigCreateMemePoolFee Success!");

    emit!(UpdatePoolConfigCreateMemePoolFeeEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
        create_meme_pool_fee: params.create_meme_pool_fee,
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigListToRaydiumFeeParams {
    pub list_to_raydium_fee: u64,
}

pub fn update_pool_config_list_to_raydium_fee(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigListToRaydiumFeeParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(ctx.accounts.config_admin.to_account_info().key(), pool_config.config_admin.key());
    
    pool_config.update_pool_config_list_to_raydium_fee(params.list_to_raydium_fee)?;

    msg!("UpdatePoolConfigListToRaydiumFee Success!");

    emit!(UpdatePoolConfigListToRaydiumFeeEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
        list_to_raydium_fee: params.list_to_raydium_fee,
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigListAdminParams {
    pub new_list_admin: Pubkey,
    pub old_list_admin: Pubkey,
}

pub fn update_pool_config_list_admin(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigListAdminParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(ctx.accounts.config_admin.to_account_info().key(), pool_config.config_admin.key());
    
    pool_config.update_pool_config_list_admin(params.new_list_admin.key())?;

    msg!("UpdatePoolConfigListAdmin Success!");

    emit!(UpdatePoolConfigListAdminEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
        new_list_admin: params.new_list_admin.key(),
        old_list_admin: params.old_list_admin.key(),
    });

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct UpdatePoolConfigFeeVaultParams {
    pub new_fee_vault: Pubkey,
    pub old_fee_vault: Pubkey,
}

pub fn update_pool_config_fee_vault(ctx: Context<UpdatePoolConfig>, params: UpdatePoolConfigFeeVaultParams) -> Result<()> {
    let mut pool_config = ctx.accounts.pool_config.load_mut()?;
    require_keys_eq!(ctx.accounts.config_admin.to_account_info().key(), pool_config.config_admin.key());
    
    pool_config.update_pool_config_fee_vault(params.new_fee_vault.key())?;

    msg!("UpdatePoolConfigFeeVault Success!");

    emit!(UpdatePoolConfigFeeVaultEvent {
        config_admin: ctx.accounts.config_admin.to_account_info().key(),
        pool_config: ctx.accounts.pool_config.to_account_info().key(),
        new_fee_vault: params.new_fee_vault.key(),
        old_fee_vault: params.old_fee_vault.key(),
    });

    Ok(())
}