use anchor_lang::{
    prelude::*,
    solana_program::{self, system_instruction},
};
use anchor_spl::{
    associated_token::AssociatedToken, token::spl_token, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

#[derive(Accounts)]
pub struct ProxyInitialize<'info> {
    //  contact to https://t.me/wizardev
    
    /// CHECK: Safe. The associated token program.
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account[mut]]
    pub payer: Signer<'info>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

pub fn proxy_initialize(
    ctx: Context<ProxyInitialize>,
    sqrt_price_x64: u128,
    open_time: u64,
) -> Result<()> {
    msg!("Transfer to Op Address");

    //  contact to https://t.me/wizardev
}
