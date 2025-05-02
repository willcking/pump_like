pub mod constants;
pub mod error;
pub mod util;
pub mod states;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("");

#[program]

pub mod pump_program {
    use super::*;

    pub fn create_meme_and_pool(ctx: Context<CreateMemeAndPool>, init_token_params: InitTokenParams) -> Result<()> {
        instructions::create_meme_and_pool(ctx, init_token_params)
    }

    pub fn buy_meme(ctx: Context<BuyMeme>, amount: u64) -> Result<()> {
        instructions::buy_meme(ctx, amount)
    }

    pub fn sell_meme(ctx: Context<SellMeme>, amount: u64) -> Result<()> {
        instructions::sell_meme(ctx, amount)
    }

    

}