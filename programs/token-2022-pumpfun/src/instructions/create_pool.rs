use anchor_lang::{
    prelude::*,
    solana_program::{
        program::invoke_signed,
        system_instruction::{self, transfer},
    },
};
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{TokenInterface , TokenAccount , Mint}
};

use crate::{
    states::{BondingCurve, InitializeConfiguration},
    FEE_SEED,
};

#[derive(Accounts)]
pub struct CreatePool<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **
}

impl<'info> CreatePool<'info> {
    pub fn process(&mut self, fee_lamports: u64) -> Result<()> {
        msg!(
            "Sent Create Fee to Fee Wallet : {} Sol ",
            ((fee_lamports as f32) / (1_000_000_000 as f32))
        );

        let transfer_instruction = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            fee_lamports,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.payer.to_account_info(),
                self.fee_account.clone(),
                self.system_program.to_account_info(),
            ],
            &[],
        )?;

        &self.bonding_curve.init(
            self.mint_addr.supply,
            self.global_configuration.initial_virtual_sol,
        );

        Ok(())
    }
}
