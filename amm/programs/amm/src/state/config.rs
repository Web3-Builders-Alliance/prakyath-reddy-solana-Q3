use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64,
    pub has_authority: bool,
    pub mint_x: InterfaceAccount,
    pub mint_y: InterfaceAccount,
    pub fee: u16,
    pub locked: bool,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub lp_bump: u8,
}

impl Config {
    pub const LEN: usize = 8 + U64_L + (PUBKEY_L * 2) + U16_L + (BOOL_L * 2) + (U8_L * 3);

    pub fn init(
        &mut self,
        seed: u64,
        has_authority: bool,
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee: u16,
        auth_bump: u8,
        config_bump: u8,
        lp_bump: u8,
    ) -> Result<()> {
        self.seed = seed;
        self.has_authority = has_authority;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.fee = fee;
        self.locked = false;
        self.auth_bump = config_bump;
        self.config_bump = config_bump;
        self.lp_bump = lp_bump;
        Ok(())
    }
}
