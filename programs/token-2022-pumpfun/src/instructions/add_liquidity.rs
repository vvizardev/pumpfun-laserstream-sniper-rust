use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface , transfer_checked, TransferChecked},
};

use crate::states::{BondingCurve, InitializeConfiguration};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    //  **
    //  **  contact on https://t.me/wizardev
    //  **
}

impl<'info> AddLiquidity<'info> {
    pub fn process(&mut self, token_amount: u64, raydium_token_amount: u64) -> Result<()> {
        // Create the transfer instruction

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
            token_amount,
            self.mint_addr.decimals,
        )?;

        msg!(
            "Add liquidity virtual {} sol , {} token ",
            self.global_configuration.initial_virtual_sol,
            token_amount
        );

        self.bonding_curve.real_token_reserves += token_amount;
        self.bonding_curve.raydium_token += raydium_token_amount;
        self.bonding_curve.virtual_token_reserves += token_amount;

        Ok(())
    }
}
