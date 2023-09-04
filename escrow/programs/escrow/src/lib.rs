use anchor_lang::prelude::*;

declare_id!("E5K3rtGD7nEvSqHTxeNfEChvcrd3xiKUkKNY9zQWam1K");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
