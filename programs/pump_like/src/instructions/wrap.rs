use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};
use anchor_spl::token;
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};

use crate::states::PoolState;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct WrapSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = native_mint,
        associated_token::authority = owner,
    )]
    pub wsol_account: Box<TokenInterface<'info, TokenAccount>>,

    #[account(mut)]
    pub native_mint: Box<TokenInterface<'info, Mint>>,

    #[account(mut)]
    pub pool: AccountLoader<'info, PoolState>,

    #[account(
        mut,
        associated_token::mint = native_mint,
        associated_token::authority = pool,
    )]
    pub wsol_vault: Box<TokenInterface<'info, TokenAccount>>,

    #[account(
        address = associated_token::ID,
    )]
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
    msg!("Wrapping SOL");

    let wsol_amount: u64;
    let user_balance = ctx.accounts.wsol_account.amount;
    msg!("Desired WSOL amount in lamports: {:?}", amount);
    msg!("User WSOL balance in lamports: {:?}", user_balance);
    if user_balance >= amount {
        msg!("Enough WSOL in token account.");
        return Ok(());
    }
    wsol_amount = amount - user_balance;
    msg!("WSOL amount to wrap in lamports: {:?}", wsol_amount);
    if ctx.accounts.owner.lamports() < wsol_amount {
        return err!(ErrorCode::InsufficientBalance);
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.wsol_account.to_account_info(),
            },
        ),
        wsol_amount,
    )?;

    token::sync_native(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::SyncNative {
            account: ctx.accounts.wsol_account.to_account_info(),
        },
    ))?;

    Ok(())
}