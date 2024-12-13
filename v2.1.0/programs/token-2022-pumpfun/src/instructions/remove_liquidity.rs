use std::ops::{Div, Mul};

use anchor_lang::{
    prelude::*,
    solana_program::{native_token::LAMPORTS_PER_SOL, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{spl_token, Token},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        init,
        payer = payer,
        associated_token::mint = native_mint,
        associated_token::authority = op_address,
        associated_token::token_program = spl_token_program,
    )]
    pub op_native_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK:
    #[account(mut)]
    pub creator: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_addr,
        associated_token::authority = op_address,
        associated_token::token_program = token_program,
    )]
    pub op_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub op_address: AccountInfo<'info>,

    pub native_mint: Box<InterfaceAccount<'info, Mint>>,
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

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub spl_token_program: Program<'info, Token>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveLiquidity<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        msg!("Transfer 0.5 Sol to Creator : ");

        // Perform SOL transfer from sol_pool to user_wallet
        let transfer_instruction_creator = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.creator.to_account_info().key(),
            LAMPORTS_PER_SOL.div(2_u64),
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction_creator,
            &[
                self.sol_pool.to_account_info(),
                self.creator.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[&self.mint_addr.key().to_bytes(), b"sol_pool", &[bump]]],
        )?;

        msg!("Transfer 1.0433 Sol to Op_Address for creating fee : ");

        // Perform SOL transfer from sol_pool to user_wallet
        let transfer_instruction_op_addr = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.op_address.to_account_info().key(),
            1.0433_f32.mul(LAMPORTS_PER_SOL as f32) as u64,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction_op_addr,
            &[
                self.sol_pool.to_account_info(),
                self.op_address.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[&self.mint_addr.key().to_bytes(), b"sol_pool", &[bump]]],
        )?;

        msg!(
            "Withdraw : {} token , {} sol ",
            self.token_pool.amount.div(LAMPORTS_PER_SOL) as f32,
            self.sol_pool.lamports().div(LAMPORTS_PER_SOL) as f32,
        );

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.sol_pool.to_account_info(),
                    from: self.token_pool.to_account_info(),
                    to: self.op_ata.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
                &[&[
                    &self.mint_addr.key().to_bytes(), // Mint address seed
                    b"sol_pool",
                    &[bump], // Constant seed
                ]],
            ),
            self.token_pool.amount,
            self.mint_addr.decimals,
        )?;

        // Perform SOL transfer from sol_pool to user_wallet
        let transfer_instruction = system_instruction::transfer(
            &self.sol_pool.to_account_info().key(),
            &self.op_native_ata.to_account_info().key(),
            self.sol_pool.lamports(),
        );

        // Invoke the transfer
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                self.sol_pool.to_account_info(),
                self.op_native_ata.to_account_info(),
                self.system_program.to_account_info(),
            ],
            &[&[&self.mint_addr.key().to_bytes(), b"sol_pool", &[bump]]],
        )?;

        // Sync native tokens to ensure proper balance
        let sync_native_ix =
            spl_token::instruction::sync_native(&spl_token::id(), &self.op_native_ata.key())?;

        anchor_lang::solana_program::program::invoke_signed(
            &sync_native_ix,
            &[
                self.op_native_ata.to_account_info(),
                self.token_program.to_account_info(),
            ],
            &[&[&self.mint_addr.key().to_bytes(), b"sol_pool", &[bump]]],
        )?;

        Ok(())
    }
}
