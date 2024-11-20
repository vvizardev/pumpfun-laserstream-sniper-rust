use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{self, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken, token::{self, spl_token, Token}, token_2022::spl_token_2022, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};


use crate::{
    states::{BondingCurve, InitializeConfiguration},
};


#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **
}

impl<'info> RemoveLiquidity<'info> {
    pub fn process(&mut self , sol_bump : u8) -> Result<()> {
        msg!("sol_pool owner: {:?}", self.sol_pool.owner);
        msg!("user_wallet owner: {:?}", self.user_wallet.owner);
        // Log account balances for debugging

        msg!(
            "Sol amount for DEX after BC: {}, Raydium token: {}",
            self.global_configuration.sol_amount_for_dex_after_bc,
            self.bonding_curve.raydium_token
        );

        // Perform SOL transfer from sol_pool to user_wallet
        let transfer_instruction = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.user_token_pc.to_account_info().key(),
            self.global_configuration.sol_amount_for_dex_after_bc,
        );

        // Invoke the transfer
        solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.sol_pool.to_account_info(),
                self.user_token_pc.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[
                &self.amm_coin_mint.key().to_bytes(),
                b"sol_pool",
                &[sol_bump],
            ]],
        )?;

        // Sync native tokens to ensure proper balance
        let sync_native_ix =
            spl_token::instruction::sync_native(&spl_token::id(), &self.user_token_pc.key)?;
        anchor_lang::solana_program::program::invoke_signed(
            &sync_native_ix,
            &[
                self.user_token_pc.to_account_info(),
                self.token_program.to_account_info(),
            ],
            &[&[
                &self.amm_coin_mint.key().to_bytes(),
                b"sol_pool",
                &[sol_bump],
            ]],
        )?;

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.sol_pool.to_account_info(),
                    from: self.token_pool.to_account_info(),
                    to: self.user_token_coin.to_account_info(),
                    mint: self.amm_coin_mint.to_account_info(),
                },&[&[
                    &self.amm_coin_mint.key().to_bytes(), // Mint address seed
                    b"sol_pool",
                    &[sol_bump], // Constant seed
                ]],
            ),
            self.bonding_curve.raydium_token,
            self.amm_coin_mint.decimals,
        )?;

        Ok(())
    }
}
