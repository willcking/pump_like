use crate::constants::meme::TOTAL_SUPPLY;
use crate::error::ErrorCode;
use crare::states::*;
use crate::util::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct SellMeme<'info> {
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub fee_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_meme_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub pool_config: AccountLoader<'info, PoolConfig>,

    #[account(mut)]
    pub meme_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn sell_meme(ctx: Context<SellMeme>, amount: u64) -> Result<()> {
    let pool_info = ctx.accounts.pool_state.load_mut()?;
    let pool_config = ctx.accounts.pool_config.load()?;

    require_keys_eq!(ctx.accounts.pool_config.key(), ctx.accounts.pool_config.key());
    require!(!pool_info.get_is_ready_to_list(), ErrorCode::PoolReadyToList);

    let fee_account = pool_config.fee_vault;    
    require_keys_eq!(ctx.accounts.fee_account.key(), fee_account.key());

    let pool_meme_amount = ctx.accounts.meme_vault.amount.clone();

       // Transfer tokens from user to pool meme vault
       anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_meme_ata.to_account_info().clone(),
                to: ctx.accounts.meme_vault.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        user_input_amount,
    )?;

    // calculate out sol amount
    let pool_sol_amount = ctx
        .accounts
        .pool_state
        .get_lamports()
        .checked_sub(pool_info.get_rent_amount())
        .unwrap();
    let out_sol_amount = calc::calc_sell_amount(
        TOTAL_SUPPLY_AMOUNT,
        pool_left_token_amount,
        pool_sol_amount,
        user_input_amount,
    );
    msg!("out_sol_amount {}:", out_sol_amount);

    // calc fee amount
    let fee_amount = out_sol_amount
        .checked_mul(pool_config.swap_fee.into())
        .unwrap()
        .checked_div(pool_config.swap_fee_denominator.into())
        .unwrap();

    let pure_sol_out_amount = out_sol_amount.checked_sub(fee_amount).unwrap();
    if pure_sol_out_amount > pool_sol_amount {
        return err!(ErrorCode::InsufficientBalance);
    }

    // transfer sol
    ctx.accounts
        .pool_state
        .to_account_info()
        .sub_lamports(out_sol_amount)?;
    ctx.accounts
        .fee_account
        .to_account_info()
        .add_lamports(fee_amount)?;
    ctx.accounts
        .user
        .to_account_info()
        .add_lamports(pure_sol_out_amount)?;

    Ok(())
}