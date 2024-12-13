use anchor_lang::prelude::*;

#[event]
pub struct BondingCurveCompleted {
    pub mint_addr: Pubkey,
    pub user_ata: Pubkey,
    pub sol_pool: Pubkey,
    pub token_pool: Pubkey,
}
