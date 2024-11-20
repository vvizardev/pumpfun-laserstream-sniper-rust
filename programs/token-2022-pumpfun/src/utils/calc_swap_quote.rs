use std::ops::{Div, Mul};

use anchor_lang::{prelude::msg, solana_program::native_token::LAMPORTS_PER_SOL};

pub fn calc_swap_quote(
    in_amount: u64,
    raydium_sol: u64,
    raydium_token: u64,
    input_sol: bool,
) -> u64 {

    if input_sol {
        ((in_amount as f64)
            .div(raydium_sol as f64)
            .mul(raydium_token as f64)) as u64
    } else {
        ((in_amount as f64)
            .div(raydium_token as f64)
            .mul(raydium_sol as f64)) as u64
    }
}
