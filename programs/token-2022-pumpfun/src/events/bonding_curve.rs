use anchor_lang::prelude::*;


#[event]
pub struct BondingCurveCompleted {
    pub mintAddr: Pubkey,
}