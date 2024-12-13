use crate::{
    states::{BondingCurve, InitializeConfiguration},
    SUPPLY, CreateMintAccountArgs,
};
use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, program::invoke, system_instruction},
    system_program::{self},
};


#[derive(Accounts)]
#[instruction(args: CreateMintAccountArgs)]
pub struct Create<'info> {
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Box<Account<'info, InitializeConfiguration>>,

    #[account(
        init,
        payer = payer,
        seeds =[ &mint_addr.key().to_bytes() , BondingCurve::POOL_SEED_PREFIX ],
        space = BondingCurve::SIZE,
        bump
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    /// CHECK:
    #[account(mut)]
    pub mint_addr: Signer<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ &mint_addr.key().to_bytes() , b"sol_pool".as_ref() ],
        bump
    )]
    pub sol_pool: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_pool: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    fn initialize_token_metadata(
        &self,
        name: String,
        symbol: String,
        uri: String,
    ) -> ProgramResult {
       
        Ok(())
    }
}
