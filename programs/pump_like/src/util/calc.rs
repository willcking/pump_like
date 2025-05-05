use anchor_lang::prelude::*;
use crate::constants::swap::{NUM1, NUM2, NUM3};

pub fn calc_buy_meme_amount(
    total_meme_amount: u64,
    pool_meme_amount: u64,
    pool_sol_amount: u64,
    input_sol_amount: u64,
) -> u64 {
    let b = total_meme_amount
        .checked_sub(pool_meme_amount)
        .unwrap();

    let coefficient1 = pool_sol_amount.checked_add(NUM3).unwrap();
    let coefficient2 = input_sol_amount.checked_add(coefficient1).unwrap();
    let coefficient3 = NUM2
        .checked_mul(1_000_000)
        .unwrap()
        .checked_div(coefficient2.checked_div(1_000).unwrap())
        .unwrap()
        .checked_mul(1_000_000_000)
        .unwrap();

    let a = NUM1.checked_sub(coefficient3).unwrap();
    let result = a.checked_sub(b).unwrap();

    return result;
}