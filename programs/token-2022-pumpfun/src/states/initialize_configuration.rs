use anchor_lang::prelude::*;

use crate::InitializeConfigurationParam;

#[account]
pub struct InitializeConfiguration {
    pub swap_fee : f32,                                       //  swap percentage
    pub bonding_curve_limitation : u64,                       //  bonding curve limitation
    pub sol_amount_for_dex_after_bc : u64,                    //  sol percentage which moves to dex after bonding curve 
    pub sol_amount_for_pumpfun_after_bc : u64,                //  sol percentage which moves to pumpfun after bonding curve 
    pub sol_amount_for_token_creator_after_bc : u64,          //  sol percentage which moves to pumpfun after bonding curve 
    pub initial_virtual_sol : u64,          //  sol percentage which moves to pumpfun after bonding curve 
}

impl InitializeConfiguration {
    pub const SIZE: usize = 48;  // Size of the struct
    pub const SEEDS: &'static [u8] = b"global_config";  // Size of the struct

    pub fn set_value (&mut self , param : InitializeConfigurationParam) -> Result<()> {

        self.bonding_curve_limitation =  param.bonding_curve_limitation;
        self.sol_amount_for_dex_after_bc =  param.sol_amount_for_dex_after_bc;
        self.sol_amount_for_pumpfun_after_bc =  param.sol_amount_for_pumpfun_after_bc;
        self.sol_amount_for_token_creator_after_bc =  param.sol_amount_for_token_creator_after_bc;
        self.swap_fee =  param.swap_fee;
        self.initial_virtual_sol =  param.initial_virtual_sol;

        Ok(())
    }
}