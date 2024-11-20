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
    events::BondingCurveCompleted,
    states::{BondingCurve, InitializeConfiguration},
    utils::calc_swap_quote,
};

#[derive(Accounts)]
pub struct Buy<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **
}

impl<'info> Buy<'info> {
    pub fn process(&mut self, in_amount: u64, bump: u8) -> Result<()> {
        let estimated_out_token = calc_swap_quote(
            in_amount,
            self.global_configuration.bonding_curve_limitation.div(100_000),
            self.bonding_curve.raydium_token.div(100_000),
            true,
        );

        msg!(
            "Buy in_amount {}  => {} ",
            in_amount.clone(),
            estimated_out_token.clone()
        );

        let transfer_instruction = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.sol_pool.to_account_info().key(),
            (in_amount as f32 * (100.0 - self.global_configuration.swap_fee.clone()) / 100.0) as u64,
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
            (in_amount as f32 * (self.global_configuration.swap_fee.clone()) / 100.0) as u64,
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
                },&[&[
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
            in_amount.clone().div(LAMPORTS_PER_SOL),
            estimated_out_token.div(10_u64.pow(self.mint_addr.decimals as u32))
        );

        self.bonding_curve.real_sol_reserves += in_amount;
        self.bonding_curve.virtual_sol_reserves += in_amount;
        self.bonding_curve.real_token_reserves -= estimated_out_token;
        self.bonding_curve.virtual_token_reserves -= estimated_out_token;

        msg!(
            "{} , {}",
            self.bonding_curve.real_sol_reserves,
            self.global_configuration.bonding_curve_limitation
        );

        if self.bonding_curve.real_sol_reserves > self.global_configuration.bonding_curve_limitation
        {
            self.bonding_curve.complete = true;
            emit!(BondingCurveCompleted {
                mintAddr: self.mint_addr.key()
            })
        }

        Ok(())
    }
}
