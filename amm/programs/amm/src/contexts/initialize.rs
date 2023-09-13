use crate::{errors::AmmError, state::Config};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = auth
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: this is fine.
    #[account(
        seeds = [b"auth", config.key().as_ref()],
        bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        space = Config::LEN,
        bump
    )]
    pub config: Account<'info, Config>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        _seed: u64,
        fee: u16,
        _authority: Option<Pubkey>,
    ) -> Result<()> {
        if fee > 10000 {
            return AmmError::InvalidFee;
        }
        let (auth_bump, config_bump, lp_bump) = (
            *bumps.get("auth")?,
            *bumps.get("config")?,
            *bumps.get("mint_lp")?,
        );
        self.config.init(
            _seed,
            _authority,
            self.mint_x.key(),
            self.mint_y.key(),
            fee,
            auth_bump,
            config_bump,
            lp_bump,
        );
        Ok(())
    }
}
