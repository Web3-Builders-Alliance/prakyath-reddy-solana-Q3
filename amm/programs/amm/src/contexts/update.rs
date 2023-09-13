use crate::errors::AmmError;
use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"config", authority.key().as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, locked: bool) -> Result<()> {
        match self.config.authority {
            Some(a) => {
                require_keys_eq!(self.authority.key, a.key(), AmmError::InvalidAuthority)
            }
            None => return err!(AmmError::NoAuthoritySet),
        }
        if self.config.has_aithority == false {
            return err!(AmmError::NoAuthoritySet);
        }
        self.config.locked = locked;
        Ok(())
    }
}
