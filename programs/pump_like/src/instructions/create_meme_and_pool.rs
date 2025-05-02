use crate::constants::meme::{DECIMALS, POOL_STATE_SEED};

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateMemeAndPool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = PoolState::LEN,
        seeds = [POOL_STATE_SEED, mint.key().as_ref()],
        bump,
    )]
    pub pool_state: AccountLoader<'info, PoolState >,

    pub mint: Account<'info, Mint>,

    pub mint_config: Account<'info, MintConfig>,

    pub meme_vault: Account<'info, TokenAccount>,

    pub pool_config: AccountLoader<'info, PoolConfig>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
}

