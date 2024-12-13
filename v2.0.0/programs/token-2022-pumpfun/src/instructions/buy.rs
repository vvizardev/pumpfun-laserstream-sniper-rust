use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    errors::RaydiumPumpfunError,
    events::BondingCurveCompleted,
    states::{BondingCurve, InitializeConfiguration},
};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    #[account(
        mut,
        seeds =[ &mint_addr.key().to_bytes() , BondingCurve::POOL_SEED_PREFIX ],
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

impl<'info> Buy<'info> {
    pub fn process(&mut self, sol_input_amt: u64, expected_amt: u64, bump: u8) -> Result<()> {
        let estimated_out_token = ((sol_input_amt as f64)
            .div((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64)
            .mul(self.bonding_curve.token_reserves as f64))
            as u64;

        msg!("{} > {}", estimated_out_token, expected_amt);
        msg!(
            "Buy : {} Sol => {} Token ( Price : {} )",
            sol_input_amt.div(LAMPORTS_PER_SOL),
            estimated_out_token.div(LAMPORTS_PER_SOL),
            ((self.bonding_curve.init_virtual_sol + self.bonding_curve.sol_reserves) as f64)
                .div(self.bonding_curve.token_reserves as f64)
        );

        if estimated_out_token < expected_amt {
            err!(RaydiumPumpfunError::SlippageExcceded).unwrap()
        }

        let transfer_instruction = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.sol_pool.to_account_info().key(),
            sol_input_amt as u64,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                self.payer.to_account_info(),
                self.sol_pool.clone(),
                self.system_program.to_account_info(),
            ],
        )?;

        let transfer_instruction_fee = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            (sol_input_amt as f32 * (self.global_configuration.swap_fee.clone()) / 100.0) as u64,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction_fee,
            &[
                self.payer.to_account_info(),
                self.fee_account.clone(),
                self.system_program.to_account_info(),
            ],
        )?;

        msg!(
            " token balance : {} , {}",
            self.token_pool.amount,
            estimated_out_token.clone()
        );

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.sol_pool.to_account_info(),
                    from: self.token_pool.to_account_info(),
                    to: self.user_ata.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
                &[&[
                    &self.mint_addr.key().to_bytes(), // Mint address seed
                    b"sol_pool",
                    &[bump], // Constant seed
                ]],
            ),
            estimated_out_token,
            self.mint_addr.decimals,
        )?;

        msg!(
            "Buy Token {} sol => {} token ",
            sol_input_amt.clone().div(LAMPORTS_PER_SOL),
            estimated_out_token.div(10_u64.pow(self.mint_addr.decimals as u32))
        );

        msg!("Before Bonding Curve : {:#?}", self.bonding_curve.get());

        let _ = self.bonding_curve.sol_reserves += sol_input_amt;
        let _ = self.bonding_curve.token_reserves -= estimated_out_token;
        let _ = self.bonding_curve.update();

        if self.bonding_curve.sol_reserves > self.global_configuration.bonding_curve_limitation {
            emit!(BondingCurveCompleted {
                mint_addr: self.mint_addr.key(),
                user_ata: self.user_ata.key(),
                sol_pool: self.sol_pool.key(),
                token_pool: self.token_pool.key(),
            })
        }

        Ok(())
    }
}
