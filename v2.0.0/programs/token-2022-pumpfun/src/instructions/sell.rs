use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface , transfer_checked, TransferChecked},
};
use crate::{
    errors::RaydiumPumpfunError,
    states::{BondingCurve, InitializeConfiguration},
};


#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    #[account(
        mut,
        seeds = [ &mint_addr.key().to_bytes() , BondingCurve::POOL_SEED_PREFIX ],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    pub mint_addr: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_addr,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ &mint_addr.key().to_bytes() , b"sol_pool".as_ref() ],
        bump
    )]
    pub sol_pool: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = mint_addr,
        associated_token::authority = sol_pool,
        associated_token::token_program = token_program,
    )]
    pub token_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK:
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Sell<'info> {
    pub fn process(&mut self, in_token_amount: u64, expected_amt : u64 ,  bump: u8) -> Result<()> {
        let estimated_out_sol = ((in_token_amount as f64).mul((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64).div(self.bonding_curve.token_reserves as f64)) as u64;

        msg!("{} > {}" , estimated_out_sol , expected_amt);
        msg!(
            "Sell : {} Token => {} Sol ( Price : {} )",
            in_token_amount.div(LAMPORTS_PER_SOL),
            estimated_out_sol.div(LAMPORTS_PER_SOL),
            ((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64).div(self.bonding_curve.token_reserves as f64)
        );

        if estimated_out_sol < expected_amt {
            err!(RaydiumPumpfunError::SlippageExcceded).unwrap()
        }

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.payer.to_account_info(),
                    from: self.user_ata.to_account_info(),
                    to: self.token_pool.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
            ),
            in_token_amount,
            self.mint_addr.decimals,
        )?;

        msg!(
            "Sell : {} token => {} sol ",
            in_token_amount,
            estimated_out_sol
        );

        let transfer_instruction = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.payer.to_account_info().key(),
            estimated_out_sol as u64,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.sol_pool.clone(),
                self.payer.to_account_info(),
                self.system_program.to_account_info(),
            ],&[&[
                &self.mint_addr.key().to_bytes(), // Mint address seed
                b"sol_pool",
                &[bump], // Constant seed
            ]],
        )?;
        let transfer_instruction_fee = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            (estimated_out_sol as f32 * ( self.global_configuration.swap_fee.clone()) / 100.0) as u64,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction_fee,
            &[
                self.payer.to_account_info(),
                self.fee_account.to_account_info(),
                self.system_program.to_account_info(),
            ],&[&[
                &self.mint_addr.key().to_bytes(), // Mint address seed
                b"sol_pool",
                &[bump], // Constant seed
            ]],
        )?;

        msg!("Before Bonding Curve : {:#?}", self.bonding_curve.get());

        let _ = self.bonding_curve.sol_reserves -= estimated_out_sol;
        let _ = self.bonding_curve.token_reserves += in_token_amount;
        let _ = self.bonding_curve.update();

        Ok(())
    }
}
