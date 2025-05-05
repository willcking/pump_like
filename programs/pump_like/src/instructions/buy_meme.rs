use crate::constants::meme::TOTAL_SUPPLY_AMOUNT;
use crate::error::ErrorCode;
use crate::{constants, states::*, util::*};
use crate::constants::POOL_SEED;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use solana_program::{Program, system_instruction}

#[derive(Accounts)]
pub struct BuyMeme<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_meme_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [POOL_SEED, mint.key().as_ref()],
        bump
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub meme_vault: Box< Account<'info, TokenAccount>>,

    #[account(mut)]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    #[account(mut)]
    pub fee_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
 
pub fn buy_meme(ctx: Context<BuyMeme>, amount: u64) -> Result<()> {
    let pool_config = ctx.accounts.pool_config.load()?.clone();
    let pool_state = &mut ctx.accounts.pool_state.load()?.clone();
    
    require_keys_eq!(ctx.accounts.pool_config.key(), pool_state.pool_config);
    require!(pool_state.is_ready_to_list, ErrorCode::PoolNotReadyToList);
    require_keys_eq!(ctx.accounts.fee_account.key(), pool_config.fee_vault);
    
    let fee_amount = amount
        .checked_mul(pool_config.swap_fee.into())
        .unwrap()
        .checked_div(pool_config.swap_fee_denominator.into())
        .unwrap();
    let transfer_amount = amount
        .checked_sub(fee_amount)
        .unwrap();

    let pool_sol_balance = ctx.accounts.pool_state
        .get_lamports()
        .checked_sub(pool.state.get_rent_amount())
        .unwrap();

    let sum_pool_sol_balance = pool_sol_balance
        .checked_add(transfer_amount)
        .unwrap();

    let transfer_instruction = &system_instruction::transfer(
        ctx.accounts.buyer.key,
        &ctx.accounts.pool_state.key(),
        transfer_amount,
    );
    program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.buyer.to_account_info().clone(),
            ctx.accounts.pool_state.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
        &[],
    )?;

    let transfer_instruction = &system_instruction::transfer(
        ctx.accounts.buyer.key,
        &ctx.accounts.fee_account.key(),
        fee_amount,
    );
    program::invoke_signed(
            &transfer_instruction,
            &[
                ctx.accounts.buyer.to_account_info().clone(),
                ctx.accounts.fee_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
            &[],
        )?;
    
    let meme_vault_amount = ctx.accounts.meme_vault.amount;
    let out_meme_amount = calc::calc_buy_meme_amount(
        TOTAL_SUPPLY_AMOUNT,
        meme_vault_amount,
        pool_sol_balance,
        transfer_amount,
    );
    if out_meme_amount > meme_vault_amount {
        return err!(ErrorCode::InsufficientBalance);
    }

    let seed = pool_state.seeds().clone();
    let singer = &[&seed[..]];
    let cpi_accounts = Transfer {
        from: ctx.accounts.meme_vault.to_account_info().clone(),
        to: ctx.accounts.buyer_meme_ata.to_account_info().clone(),
        authority: ctx.accounts.pool_state.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        singer,
    );
    transfer(cpi_context, out_meme_amount)?;

    if sum_pool_sol_balance >= constants::LISTINGS_MARKET_AMOUNT {
        ctx.accounts
            .pool_state
            .to_account_info()
            .sub_lamports(pool_config.get_list_to_ray_fee())?;
        ctx.accounts
            .fee_account
            .to_account_info()
            .add_lamports(pool_config.get_list_to_ray_fee())?;

        // Update Pool state
        let mut pool_info = ctx.accounts.pool_state.load_mut()?;
        pool_info.set_is_ready_to_list(true)?;
    }
}