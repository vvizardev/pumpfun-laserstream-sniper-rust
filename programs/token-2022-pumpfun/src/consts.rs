use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeConfigurationParam {
    pub swap_fee : f32,                                       //  swap percentage
    pub bonding_curve_limitation : u64,                       //  bonding curve limitation
    pub sol_amount_for_dex_after_bc : u64,                    //  sol percentage which moves to dex after bonding curve 
    pub sol_amount_for_pumpfun_after_bc : u64,                //  sol percentage which moves to pumpfun after bonding curve 
    pub sol_amount_for_token_creator_after_bc : u64,          //  sol percentage which moves to pumpfun after bonding curve 
    pub initial_virtual_sol : u64,          //  sol percentage which moves to pumpfun after bonding curve 
}

pub const FEE_SEED : &'static [u8] = b"pumpfun_fee";