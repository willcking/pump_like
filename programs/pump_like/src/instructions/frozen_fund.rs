use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::{PoolState, PoolConfig};
use crate::constants::meme::POOL_SEED;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct FrozenFund<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
    
    #[account(mut)]
    pub list_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [POOL_SEED.as_bytes(), mint.key().as_ref()],
        bump
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub meme_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = list_admin,
        associated_token::mint = mint,
        associated_token::authority = list_admin,
    )]
    pub list_admin_meme_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn frozen_fund(ctx: Context<FrozenFund>) -> Result<()> {
    let pool_state = ctx.accounts.pool_state.load()?;
    require_keys_eq!(pool_state.list_admin.key(), ctx.accounts.list_admin.key());

    let pool_config = ctx.accounts.pool_config.load()?;
    require_keys_eq!(pool_config.list_admin.key(), ctx.accounts.list_admin.to_account_info().key());
    require!(pool_state.is_ready_to_list, ErrorCode::NotReadyToList);


    let meme_vault_amount = ctx.accounts.meme_vault.amount;
    let sol_amount = ctx
       .accounts
       .pool_state
       .get_lamports()
       .checked_sub(pool_state.get_rent_amount())
       .unwrap();

    // transfer meme token
    let signer_seeds: &[&[&[u8]]] = &[&pool_state.seeds()];
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.meme_vault.to_account_info(),
                to: ctx.accounts.list_admin_meme_ata.to_account_info(),
                authority: ctx.accounts.pool_state.to_account_info(),
            },
            signer_seeds,
        ),
        meme_vault_amount,
     )?;
 
    // transfer sol
    ctx.accounts
         .pool_state
         .to_account_info()
         .sub_lamports(sol_amount)?;
    ctx.accounts
         .list_admin
         .to_account_info()
         .add_lamports(sol_amount)?;

    emit!(FrozenFundToAuthorityEvent {
        frozen_authority: ctx.accounts.pool_state.key(),
        meme_mint: pool_state.meme_mint,
        meme_amount: meme_vault_amount,
        sol_amount: sol_amount,
    });
 
    Ok(())
}
