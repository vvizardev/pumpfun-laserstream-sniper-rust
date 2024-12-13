use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::consts::*;
use crate::instructions::*;

declare_id!("GY4gideNhYWJLkgxDW7q9hS6U2SrKb9AmSUbJPsWhEKB");

#[program]
pub mod token_2022_pumpfun {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, param: InitializeConfigurationParam) -> Result<()> {
        ctx.accounts.process(param);
        Ok(())
    }

    pub fn create(ctx: Context<Create>, args: CreateMintAccountArgs) -> Result<()> {
        ctx.accounts.process(args);
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, in_amount: u64, expected_amt: u64) -> Result<()> {
        ctx.accounts
            .process(in_amount, expected_amt, ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn sell(ctx: Context<Sell>, in_amount: u64, expected_amt: u64) -> Result<()> {
        ctx.accounts
            .process(in_amount, expected_amt, ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
        ctx.accounts.process(ctx.bumps.sol_pool);
        Ok(())
    }

    pub fn proxy_initialize(ctx: Context<ProxyInitialize>) -> Result<()> {
        instructions::proxy_initialize(ctx)
    }
}
