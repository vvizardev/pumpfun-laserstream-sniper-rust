use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    consts::InitializeConfigurationParam, errors::RaydiumPumpfunError,
    states::InitializeConfiguration, FEE_SEED,
};

#[derive(Accounts)]
#[instruction(params : InitializeConfigurationParam)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [ b"global_config"],
        payer = payer,
        space = 8 + 48,
        bump
    )]
    pub global_configuration: Account<'info, InitializeConfiguration>,

    /// CHECK:
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, param: InitializeConfigurationParam) -> Result<()> {
        let bonding_curve_limitation = param.bonding_curve_limitation.clone();
        let sol_amount_for_dex_after_bc = param.sol_amount_for_dex_after_bc.clone();
        let sol_amount_for_pumpfun_after_bc = param.sol_amount_for_pumpfun_after_bc.clone();
        let sol_amount_for_token_creator_after_bc =
            param.sol_amount_for_token_creator_after_bc.clone();

        msg!(
            "{} : {}",
            bonding_curve_limitation,
            (sol_amount_for_dex_after_bc
                + sol_amount_for_pumpfun_after_bc
                + sol_amount_for_token_creator_after_bc)
        );

        require_eq!(
            bonding_curve_limitation,
            (sol_amount_for_dex_after_bc
                + sol_amount_for_pumpfun_after_bc
                + sol_amount_for_token_creator_after_bc),
            RaydiumPumpfunError::MissMatchingValue
        );

        self.global_configuration.set_value(param);

        Ok(())
    }
}
