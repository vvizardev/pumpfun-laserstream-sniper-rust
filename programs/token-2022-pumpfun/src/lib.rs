use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::consts::*;
use crate::instructions::*;

declare_id!("5VD3y7hTmCqoGodiFZKBXqbaCWTm3iVwuLowxfhE6b7h");

#[program]
pub mod token_2022_pumpfun {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, param: InitializeConfigurationParam) -> Result<()> {
        ctx.accounts.process(param);
        Ok(())
    }

    pub fn create_pool(ctx: Context<CreatePool>, fee_lamports: u64) -> Result<()> {
        ctx.accounts.process(fee_lamports);
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        token_amount: u64,
        raydium_token_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(token_amount, raydium_token_amount);
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, in_amount: u64) -> Result<()> {
        ctx.accounts.process(in_amount, ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn sell(ctx: Context<Sell>, in_amount: u64) -> Result<()> {
        ctx.accounts.process(in_amount, ctx.bumps.sol_pool);
        Ok(())
    }

    /// Initiazlize a swap pool
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.sol_pool);
        Ok(())
    }
}
