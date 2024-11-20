use anchor_lang::prelude::*;

#[account]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub raydium_token: u64,
    pub complete: bool,
}

impl BondingCurve {
    pub const POOL_SEED_PREFIX: &'static [u8] = b"bonding_curve";

    pub const SIZE: usize = 80;

    pub fn init(
        &mut self,
        token_total_supply: u64,
        initial_virtual_sol: u64,
    ) -> Result<()> {
        self.virtual_token_reserves = 0;
        self.virtual_sol_reserves = initial_virtual_sol;
        self.real_sol_reserves = 0;
        self.token_total_supply = token_total_supply;

        Ok(())
    }
}
